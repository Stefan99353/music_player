use actix_web::{Error, get, HttpResponse, post, web};

use crate::crawler::Crawler;
use crate::DbPool;
use crate::settings::Settings;

#[post("/update_db")]
pub async fn update_db(
    pool: web::Data<DbPool>,
    settings: web::Data<Settings>,
) -> Result<HttpResponse, Error> {
    let settings = settings.into_inner();

    let paths = settings
        .music
        .iter()
        .map(|d| String::from(&d.path))
        .collect::<Vec<String>>();

    let crawler = Crawler::new();

    crawler.start(paths, pool.into_inner());

    Ok(HttpResponse::Ok().finish())
}

#[post("/rebuild_db")]
pub async fn rebuild_db(
    _pool: web::Data<DbPool>,
    _settings: web::Data<Settings>,
) -> Result<HttpResponse, Error> {
    todo!("Rebuild database")
}


#[get("/updates")]
pub async fn get_updates(
    _pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    todo!("Return updates");
}
