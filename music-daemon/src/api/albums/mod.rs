use actix_web::{delete, Error, get, HttpResponse, post, put, web};
use serde::Deserialize;

use crate::DbPool;
use crate::models::albums::Album;

mod actions;

#[get("/{album_id}")]
pub async fn get_album(
    web::Path(album_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool!");

    let album = web::block(move || actions::find_album_by_id(album_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    match album {
        None => {
            let res =
                HttpResponse::NotFound().body(format!("No album found with id: {}", album_id));
            Ok(res)
        }
        Some(album) => Ok(HttpResponse::Ok().json(album)),
    }
}

#[get("")]
pub async fn all_albums(
    filter: web::Query<AlbumFilter>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool!");
    let filter = filter.into_inner();

    let albums = web::block(move || actions::get_albums(filter, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(albums))
}

#[put("/{album_id}")]
pub async fn update_album(
    web::Path(_album_id): web::Path<i32>,
    _album: web::Json<Album>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let _conn = pool.get().expect("Couldn't get DB connection from pool!");
    todo!("Update Album");
}

#[delete("/{album_id}")]
pub async fn delete_album(
    web::Path(_album_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let _conn = pool.get().expect("Couldn't get DB connection from pool!");
    todo!("Delete Album");
}

#[post("")]
pub async fn add_album(
    _album: web::Json<Album>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let _conn = pool.get().expect("Couldn't get DB connection from pool!");
    todo!("Add Album");
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumFilter {
    artist_id: Option<i32>,
    filter: Option<String>,
    page: Option<i32>,
    limit: Option<i32>,
}
