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
use simplelog::{Config, TerminalMode, TermLogger};

use crate::player::RodioPlayer;
use crate::settings::Settings;
use crate::ws::hub::WsHub;

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
    // Setup config and logger
    let settings = Settings::new().unwrap();
    TermLogger::init(settings.log_level, Config::default(), TerminalMode::Mixed).unwrap();
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
    let device: cpal::Device = cpal::default_host().default_output_device().unwrap();
    let (player, _stream) = RodioPlayer::new(device).unwrap();
    let player = web::Data::new(Mutex::new(player));

    // Create Actor for WebSocket Connection
    let ws_hub = web::Data::new(WsHub::new(player.clone().into_inner()).start());

    {
        let mut p = player.lock().unwrap();
        p.set_ws_connections(ws_hub.clone().into_inner());
    }

    // Start player
    RodioPlayer::start_player(player.clone().into_inner());

    // Start API Server
    info!("Starting API Server");
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
            .app_data(ws_hub.clone());

        app = app.service(
            web::scope("/api/v1/")
                // Websocket
                .service(ws::start_connection::start_connection)
                // Management
                .service(
                    web::scope("management")
                        .service(api::management::update_db)
                        .service(api::management::rebuild_db)
                        .service(api::management::get_updates),
                )
                // Artists
                .service(
                    web::scope("artists")
                        .service(api::artists::get_artist)
                        .service(api::artists::all_artists)
                        .service(api::artists::update_artist)
                        .service(api::artists::delete_artist)
                        .service(api::artists::add_artist),
                )
                // Albums
                .service(
                    web::scope("albums")
                        .service(api::albums::get_album)
                        .service(api::albums::all_albums)
                        .service(api::albums::update_album)
                        .service(api::albums::delete_album)
                        .service(api::albums::add_album),
                )
                // Tracks
                .service(
                    web::scope("tracks")
                        .service(api::tracks::get_track)
                        .service(api::tracks::all_tracks)
                        .service(api::tracks::update_track)
                        .service(api::tracks::delete_track)
                        .service(api::tracks::add_track)
                        .service(api::tracks::stream_track),
                )
                // Player
                .service(
                    web::scope("player")
                        .service(api::player::state)
                        .service(api::player::resume)
                        .service(api::player::pause)
                        .service(api::player::stop)
                        .service(api::player::next)
                        .service(api::player::prev)
                        .service(api::player::set_volume)
                        .service(api::player::get_volume),
                )
                // Queue
                .service(
                    web::scope("queue")
                        .service(api::queue::get_queue)
                        .service(api::queue::clear_queue)
                        .service(api::queue::add_to_queue)
                        .service(api::queue::add_album_to_queue)
                        .service(api::queue::add_artist_to_queue)
                        .service(api::queue::length),
                )
                // Images
                .service(
                    web::scope("images")
                        .service(api::images::get_image)
                        .service(api::images::get_album_image)
                        .service(api::images::get_album_image_id)
                        .service(api::images::get_artist_image)
                        .service(api::images::get_artist_image_id),
                ),
        )
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
