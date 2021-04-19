use actix_files::NamedFile;
use actix_web::{delete, Error, get, HttpResponse, post, put, web};
use serde::Deserialize;

use crate::DbPool;
use crate::models::tracks::Track;

mod actions;

#[get("/{track_id}")]
pub async fn get_track(
    web::Path(track_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool!");

    let track = web::block(move || actions::find_track_by_id(track_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    match track {
        None => {
            let res =
                HttpResponse::NotFound().body(format!("No track found with id: {}", track_id));
            Ok(res)
        }
        Some(track) => Ok(HttpResponse::Ok().json(track)),
    }
}

#[get("/{track_id}/stream")]
pub async fn stream_track(
    web::Path(track_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<NamedFile, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool!");

    let track = web::block(move || actions::find_track_by_id(track_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(NamedFile::open(&track.unwrap().path)?)
}

#[get("")]
pub async fn all_tracks(
    filter: web::Query<TrackFilter>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool!");
    let filter = filter.into_inner();

    let tracks = web::block(move || actions::get_tracks(filter, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(tracks))
}

#[put("/{track_id}")]
pub async fn update_track(
    web::Path(_track_id): web::Path<i32>,
    _track: web::Json<Track>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let _conn = pool.get().expect("Couldn't get DB connection from pool!");
    todo!("Update Track");
}

#[delete("/{track_id}")]
pub async fn delete_track(
    web::Path(_track_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let _conn = pool.get().expect("Couldn't get DB connection from pool!");
    todo!("Delete Track");
}

#[post("")]
pub async fn add_track(
    _track: web::Json<Track>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let _conn = pool.get().expect("Couldn't get DB connection from pool!");
    todo!("Add Track");
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackFilter {
    artist_id: Option<i32>,
    album_id: Option<i32>,
    filter: Option<String>,
    sort: Option<String>,
    order: Option<String>,
    page: Option<i32>,
    limit: Option<i32>,
}
