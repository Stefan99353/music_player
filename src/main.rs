#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

use std::sync::Mutex;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{App, guard, HttpResponse, HttpServer, web};
use actix_web::middleware::Logger;
use cpal::traits::HostTrait;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;

use crate::player::RodioPlayer;
use crate::settings::Settings;
use crate::ws::player::hub::WsPlayerHub;
use flexi_logger::{LogTarget, Duplicate, DeferredNow, Record, style, Criterion, Age, Naming, Cleanup};
use crate::ws::notifications::hub::WsNotificationHub;

mod api;
mod crawler;
mod models;
mod player;
mod schema;
mod settings;
mod paginate;
mod ws;

embed_migrations!("migrations");
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

async fn index() -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup settings
    let settings = Settings::new().expect("Error reading settings");

    // Setup logging
    setup_logging(&settings);
    debug!("Settings and Logger initialized");

    // Setup Database
    debug!("Connect to SQLite DB");
    let manager = ConnectionManager::<SqliteConnection>::new(&settings.database.file);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool.");

    // Run migrations
    info!("Running migrations");
    let conn = pool.get().expect("Couldn't get db connection from pool.");
    embedded_migrations::run(&conn).expect("Error running migrations.");


    // Create player daemon
    debug!("Create player and WS hub");
    let device: cpal::Device = cpal::default_host().default_output_device().expect("Error getting default audio device");
    let (player, _stream) = RodioPlayer::new(device).expect("Error creating player");
    let player = web::Data::new(Mutex::new(player));

    // Create Actor for Player WebSocket Connection
    let ws_player_hub = web::Data::new(WsPlayerHub::new(player.clone().into_inner()).start());
    let ws_notification_hub = web::Data::new(WsNotificationHub::new().start());

    {
        let mut p = player.lock().unwrap();
        p.set_ws_connections(ws_player_hub.clone().into_inner());
    }

    // Start player
    RodioPlayer::start_player(player.clone().into_inner());

    // Start API Server
    let server_settings = settings.clone();
    HttpServer::new(move || {
        let mut app = App::new().wrap(
            Cors::default()
                .allow_any_header()
                .allow_any_method()
                .allow_any_origin(),
        )
            .wrap(Logger::new("%r responded %s in %D ms"))
            .data(pool.clone())
            .app_data(player.clone())
            .data(server_settings.clone())
            .app_data(ws_player_hub.clone())
            .app_data(ws_notification_hub.clone());

        let mut api_scope = web::scope("/api/v1/");

        api_scope = ws::register(api_scope);
        api_scope = api::artists::register(api_scope);
        api_scope = api::albums::register(api_scope);
        api_scope = api::tracks::register(api_scope);
        api_scope = api::playlists::register(api_scope);
        api_scope = api::player::register(api_scope);
        api_scope = api::queue::register(api_scope);
        api_scope = api::management::register(api_scope);
        api_scope = api::images::register(api_scope);


        app = app.service(api_scope)
            // UI
            .service(actix_files::Files::new("/static", "static"))
            .default_service(
                web::resource("")
                    .route(web::get().to(index))
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    )
            );

        app
    })
        .bind(format!(
            "{}:{}",
            &settings.server.address, &settings.server.port
        ))?
        .run()
        .await
}

pub fn setup_logging (settings: &Settings) {
    flexi_logger::Logger::with_str(&settings.log_level)
        .log_target(LogTarget::File)
        .directory("logs")
        .create_symlink("current_run.log")
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(14)
        )
        .buffer_and_flush()
        .duplicate_to_stderr(Duplicate::Warn)
        .format_for_stderr(log_format_colored)
        .format_for_files(log_format)
        .start()
        .expect("Error creating logger");
}

pub fn log_format_colored(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let level = record.level();
    write!(
        w,
        "{} [{}]: {}",
        style(level, now.now().format("%Y-%m-%d %H:%M:%S")),
        style(level, record.level()),
        style(level, &record.args())
    )
}

pub fn log_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "{} [{}]: {}",
        now.now().format("%Y-%m-%d %H:%M:%S"),
        record.level(),
        &record.args()
    )
}
