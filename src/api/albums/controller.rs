use actix_web::{delete, get, post, put};
use actix_web::{Error, HttpResponse, web};

use crate::api::{get_db_connection, RequestFilter};
use crate::DbPool;
use crate::models::albums::Album;

use super::actions;

#[get("")]
pub async fn all_albums(
    filter: web::Query<RequestFilter>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;
    let filter = filter.into_inner();

    let items = web::block(move || actions::all_albums(filter, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(items))
}

#[post("")]
pub async fn add_album(
    new_album: web::Json<Album>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;
    let new_album = new_album.into_inner();

    let _result = web::block(move || actions::add_album(new_album, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    // TODO: Implement
    Err(HttpResponse::NotImplemented().finish().into())
}

#[get("/{album_id}")]
pub async fn get_album(
    web::Path(album_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    let album = web::block(move || Album::get(album_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    match album {
        None => { Ok(HttpResponse::NotFound().finish()) }
        Some(album) => { Ok(HttpResponse::Ok().json(album)) }
    }
}

#[put("/{album_id}")]
pub async fn update_album(
    web::Path(album_id): web::Path<i32>,
    album: web::Json<Album>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;
    let album = album.into_inner();

    let _result = web::block(move || actions::update_album(album_id, album, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    // TODO: Implement
    Err(HttpResponse::NotImplemented().finish().into())
}

#[delete("/{album_id}")]
pub async fn delete_album(
    web::Path(album_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    web::block(move || actions::delete_album(album_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    // TODO: Implement
    Err(HttpResponse::NotImplemented().finish().into())
}

#[get("/{album_id}/tracks")]
pub async fn all_tracks(
    web::Path(album_id): web::Path<i32>,
    filter: web::Query<RequestFilter>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;
    let filter = filter.into_inner();

    let items = web::block(move || actions::all_tracks(album_id, filter, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(items))
}

#[post("/{album_id}/image")]
pub async fn add_image(
    web::Path(album_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    let _result = web::block(move || actions::add_image(album_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    // TODO: Implement
    Err(HttpResponse::NotImplemented().finish().into())
}

#[delete("/{album_id}/image")]
pub async fn delete_image(
    web::Path(album_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    web::block(move || actions::delete_image(album_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    // TODO: Implement
    Err(HttpResponse::NotImplemented().finish().into())
}

