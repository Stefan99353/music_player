use actix_web::{delete, get, post, put, HttpRequest};
use actix_web::{Error, HttpResponse, web};

use crate::api::{get_db_connection, RequestFilter};
use crate::DbPool;
use crate::models::tracks::{PopulatedTrack, Track};

use super::actions;
use actix_files::NamedFile;

#[get("")]
pub async fn all_tracks(
    filter: web::Query<RequestFilter>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;
    let filter = filter.into_inner();

    let items = web::block(move || actions::all_tracks(filter, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(items))
}

#[post("")]
pub async fn add_track(
    new_track: web::Json<Track>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;
    let new_track = new_track.into_inner();

    let _result = web::block(move || actions::add_track(new_track, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    // TODO: Implement
    Err(HttpResponse::NotImplemented().finish().into())
}

#[get("/{track_id}")]
pub async fn get_track(
    web::Path(track_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    let track = web::block(move || PopulatedTrack::get(track_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    match track {
        None => { Ok(HttpResponse::NotFound().finish()) }
        Some(track) => { Ok(HttpResponse::Ok().json(track)) }
    }
}

#[put("/{track_id}")]
pub async fn update_track(
    web::Path(track_id): web::Path<i32>,
    track: web::Json<Track>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;
    let track = track.into_inner();

    let _result = web::block(move || actions::update_track(track_id, track, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    // TODO: Implement
    Err(HttpResponse::NotImplemented().finish().into())
}

#[delete("/{track_id}")]
pub async fn delete_track(
    web::Path(track_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    web::block(move || actions::delete_track(track_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    // TODO: Implement
    Err(HttpResponse::NotImplemented().finish().into())
}

#[get("/{track_id}/stream")]
pub async fn stream_track(
    web::Path(track_id): web::Path<i32>,
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    let track = web::block(move || Track::get(track_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    return if let Some(track) = track {
        NamedFile::open(track.path)?.into_response(&req)
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[get("/{track_id}/image")]
pub async fn get_image_id(
    web::Path(track_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    let image_id = web::block(move || actions::get_image_id(track_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    match image_id {
        None => { Ok(HttpResponse::NotFound().finish()) }
        Some(image_id) => { Ok(HttpResponse::Ok().json(image_id)) }
    }
}

#[post("/{track_id}/image")]
pub async fn add_image(
    web::Path(track_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    let _result = web::block(move || actions::add_image(track_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    // TODO: Implement
    Err(HttpResponse::NotImplemented().finish().into())
}

#[delete("/{track_id}/image")]
pub async fn delete_image(
    web::Path(track_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    web::block(move || actions::delete_image(track_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    // TODO: Implement
    Err(HttpResponse::NotImplemented().finish().into())
}
