use actix_web::{delete, Error, get, HttpResponse, post, put, web};
use serde::Deserialize;

use crate::DbPool;
use crate::models::artists::Artist;

mod actions;

#[get("/{artist_id}")]
pub async fn get_artist(
    web::Path(artist_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool!");

    let artist = web::block(move || actions::find_artist_by_id(artist_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    match artist {
        None => {
            let res =
                HttpResponse::NotFound().body(format!("No artist found with id: {}", artist_id));
            Ok(res)
        }
        Some(artist) => Ok(HttpResponse::Ok().json(artist)),
    }
}

#[get("")]
pub async fn all_artists(
    filter: web::Query<ArtistFilter>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool!");
    let filter = filter.into_inner();

    let artists = web::block(move || actions::get_artists(filter, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(artists))
}

#[put("/{artist_id}")]
pub async fn update_artist(
    web::Path(_artist_id): web::Path<i32>,
    _artist: web::Json<Artist>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let _conn = pool.get().expect("Couldn't get DB connection from pool!");
    todo!("Update Artist");
}

#[delete("/{artist_id}")]
pub async fn delete_artist(
    web::Path(_artist_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let _conn = pool.get().expect("Couldn't get DB connection from pool!");
    todo!("Delete Artist");
}

#[post("")]
pub async fn add_artist(
    _artist: web::Json<Artist>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let _conn = pool.get().expect("Couldn't get DB connection from pool!");
    todo!("Add Artist");
}


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistFilter {
    filter: Option<String>,
    page: Option<i32>,
    limit: Option<i32>,
}
