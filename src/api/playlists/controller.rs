use actix_web::{delete, get, post, put};
use actix_web::{Error, HttpResponse, web};

use crate::api::{get_db_connection, RequestFilter};
use crate::DbPool;
use crate::models::playlists::{Playlist, PlaylistTrack};

use super::actions;

#[get("")]
pub async fn all_playlists(
    filter: web::Query<RequestFilter>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;
    let filter = filter.into_inner();

    let items = web::block(move || actions::all_playlists(filter, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(items))
}

#[post("")]
pub async fn add_playlist(
    new_playlist: web::Json<Playlist>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;
    let new_playlist = new_playlist.into_inner();

    if new_playlist.name.is_empty() {
        return Err(HttpResponse::BadRequest().finish().into())
    }

    let playlist = web::block(move || new_playlist.insert(&conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(playlist))
}

#[get("/{playlist_id}")]
pub async fn get_playlist(
    web::Path(playlist_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    let playlist = web::block(move || Playlist::get(playlist_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    match playlist {
        None => { Err(HttpResponse::NotFound().finish().into()) }
        Some(playlist) => { Ok(HttpResponse::Ok().json(playlist)) }
    }
}

#[put("/{playlist_id}")]
pub async fn update_playlist(
    web::Path(playlist_id): web::Path<i32>,
    new_playlist: web::Json<Playlist>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;
    let new_playlist = new_playlist.into_inner();

    let playlist = web::block(move || actions::update_playlist(playlist_id, new_playlist, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(playlist))
}

#[delete("/{playlist_id}")]
pub async fn delete_playlist(
    web::Path(playlist_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    web::block(move || actions::delete_playlist(playlist_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/{playlist_id}/tracks")]
pub async fn all_tracks(
    web::Path(playlist_id): web::Path<i32>,
    filter: web::Query<RequestFilter>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;
    let filter = filter.into_inner();

    let items = web::block(move || actions::all_tracks(playlist_id, filter, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(items))
}

#[derive(serde::Deserialize)]
pub struct PlaylistTrackRequest {
    playlist_id: i32,
    track_id: i32,
}

#[post("/{playlist_id}/tracks/{track_id}")]
pub async fn add_track(
    web::Path(playlist_track_request): web::Path<PlaylistTrackRequest>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    let playlist_track = PlaylistTrack {
        playlist_id: playlist_track_request.playlist_id,
        track_id: playlist_track_request.track_id,
        inserted: None,
        updated: None
    };

    let result = web::block(move || playlist_track.insert(&conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(result))
}

#[delete("/{playlist_id}/tracks/{track_id}")]
pub async fn delete_track(
    web::Path(playlist_track_request): web::Path<PlaylistTrackRequest>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = get_db_connection(pool.into_inner())?;

    let result = web::block(move || actions::delete_track(playlist_track_request.playlist_id, playlist_track_request.track_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    if result < 1 {
        return Ok(HttpResponse::NotFound().finish());
    }

    Ok(HttpResponse::Ok().finish())
}
