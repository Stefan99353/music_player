use actix_web::{delete, Error, get, HttpResponse, post, web};
use serde::Deserialize;

use crate::api::PlayerData;
use crate::DbPool;
use crate::api::queue::actions;

#[get("")]
pub async fn all_tracks(
    player: web::Data<PlayerData>
) -> Result<HttpResponse, Error> {
    let player = player.lock().unwrap();

    let queue = player.get_queue();

    Ok(HttpResponse::Ok().json(queue))
}

#[delete("")]
pub async fn clear_queue(
    player: web::Data<PlayerData>
) -> Result<HttpResponse, Error> {
    let mut player = player.lock().unwrap();

    player.clear_queue();

    Ok(HttpResponse::Ok().finish())
}

#[get("length")]
pub async fn length(
    player: web::Data<PlayerData>
) -> Result<HttpResponse, Error> {
    let player = player.lock().unwrap();

    let length = player.get_queue().len();

    Ok(HttpResponse::Ok().json(length))
}

#[post("add")]
pub async fn add_to_queue(
    pool: web::Data<DbPool>,
    player: web::Data<PlayerData>,
    tracks: web::Json<Vec<i32>>,
    shuffle: web::Query<Shuffle>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get db connection from pool");
    let player = player.into_inner();
    let tracks = tracks.into_inner();
    let shuffle = shuffle.into_inner().shuffle;

    // use web::block to offload blocking Diesel code without blocking server thread
    web::block(move || actions::add_to_queue(&conn, player, tracks, shuffle))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        })?;

    Ok(HttpResponse::Ok().finish())
}

#[post("add/artist")]
pub async fn add_artist_to_queue(
    pool: web::Data<DbPool>,
    player: web::Data<PlayerData>,
    artist_id: web::Json<i32>,
    shuffle: web::Query<Shuffle>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get db connection from pool");
    let player = player.into_inner();
    let artist_id = artist_id.into_inner();
    let shuffle = shuffle.into_inner().shuffle;

    // use web::block to offload blocking Diesel code without blocking server thread
    web::block(move || actions::add_artist_to_queue(&conn, player, artist_id, shuffle))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        })?;

    Ok(HttpResponse::Ok().finish())
}

#[post("add/album")]
pub async fn add_album_to_queue(
    pool: web::Data<DbPool>,
    player: web::Data<PlayerData>,
    album_id: web::Json<i32>,
    shuffle: web::Query<Shuffle>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get db connection from pool");
    let player = player.into_inner();
    let album_id = album_id.into_inner();
    let shuffle = shuffle.into_inner().shuffle;

    // use web::block to offload blocking Diesel code without blocking server thread
    web::block(move || actions::add_album_to_queue(&conn, player, album_id, shuffle))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        })?;

    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug, Clone, Deserialize)]
pub struct Shuffle {
    shuffle: bool,
}
