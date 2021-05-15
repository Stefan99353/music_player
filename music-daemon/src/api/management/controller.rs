use actix::Addr;
use actix_web::{Error, get, HttpResponse, post, web};

use crate::crawler::Crawler;
use crate::DbPool;
use crate::settings::Settings;
use crate::ws::notifications::hub::WsNotificationHub;

use super::actions;

#[post("/update_db")]
pub async fn update_db(
    pool: web::Data<DbPool>,
    settings: web::Data<Settings>,
    notifications: web::Data<Addr<WsNotificationHub>>,
) -> Result<HttpResponse, Error> {
    let settings = settings.into_inner();

    let paths = settings
        .music
        .iter()
        .map(|d| String::from(&d.path))
        .collect::<Vec<String>>();

    Crawler::new(
        paths,
        pool.into_inner(),
        notifications.into_inner(),
    ).start();

    Ok(HttpResponse::Accepted().finish())
}

#[post("/rebuild_db")]
pub async fn rebuild_db(
    pool: web::Data<DbPool>,
    settings: web::Data<Settings>,
    notifications: web::Data<Addr<WsNotificationHub>>,
) -> Result<HttpResponse, Error> {
    actions::clear_db(
        pool.into_inner(),
        settings.into_inner(),
        notifications.into_inner(),
    );

    Ok(HttpResponse::Accepted().finish())
}


#[get("/updates")]
pub async fn all_updates(
    _pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    todo!("Return updates");
}
