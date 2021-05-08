use actix_web::{delete, get, post, put};
use actix_web::{Error, HttpResponse, web};

use crate::api::{get_db_connection, RequestFilter};
use crate::DbPool;
use crate::models::artists::Artist;

use super::actions;

#[get("")]
pub async fn all_artists(
    filter: web::Query<RequestFilter>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;
    let filter = filter.into_inner();

    let items = web::block(move || actions::all_artists(filter, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(items))
}

#[post("")]
pub async fn add_artist(
    new_artist: web::Json<Artist>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;
    let new_artist = new_artist.into_inner();

    let _result = web::block(move || actions::add_artist(new_artist, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    // TODO: Implement
    Err(HttpResponse::NotImplemented().finish().into())
}

#[get("/{artist_id}")]
pub async fn get_artist(
    web::Path(artist_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    let artist = web::block(move || Artist::get(artist_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    match artist {
        None => { Ok(HttpResponse::NotFound().finish()) }
        Some(artist) => { Ok(HttpResponse::Ok().json(artist)) }
    }
}

#[put("/{artist_id}")]
pub async fn update_artist(
    web::Path(artist_id): web::Path<i32>,
    artist: web::Json<Artist>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;
    let artist = artist.into_inner();

    let _result = web::block(move || actions::update_artist(artist_id, artist, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    // TODO: Implement
    Err(HttpResponse::NotImplemented().finish().into())
}

#[delete("/{artist_id}")]
pub async fn delete_artist(
    web::Path(artist_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    web::block(move || actions::delete_artist(artist_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    // TODO: Implement
    Err(HttpResponse::NotImplemented().finish().into())
}

#[get("/{artist_id}/albums")]
pub async fn all_albums(
    web::Path(artist_id): web::Path<i32>,
    filter: web::Query<RequestFilter>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;
    let filter = filter.into_inner();

    let items = web::block(move || actions::all_albums(artist_id, filter, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(items))
}

#[get("/{artist_id}/tracks")]
pub async fn all_tracks(
    web::Path(artist_id): web::Path<i32>,
    filter: web::Query<RequestFilter>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;
    let filter = filter.into_inner();

    let items = web::block(move || actions::all_tracks(artist_id, filter, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(items))
}

#[post("/{artist_id}/image")]
pub async fn add_image(
    web::Path(artist_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    let _result = web::block(move || actions::add_image(artist_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    // TODO: Implement
    Err(HttpResponse::NotImplemented().finish().into())
}

#[delete("/{artist_id}/image")]
pub async fn delete_image(
    web::Path(artist_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    web::block(move || actions::delete_image(artist_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    // TODO: Implement
    Err(HttpResponse::NotImplemented().finish().into())
}
