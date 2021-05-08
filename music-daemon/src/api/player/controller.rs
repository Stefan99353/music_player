use std::time::Duration;

use actix_web::{Error, get, HttpResponse, post, web};
use serde::Deserialize;

use crate::api::PlayerData;

#[get("")]
pub async fn state(player: web::Data<PlayerData>) -> Result<HttpResponse, Error> {
    let player = player.lock().unwrap();

    let res = player.get_state();

    Ok(HttpResponse::Ok().json(res))
}

#[post("resume")]
pub async fn resume(player: web::Data<PlayerData>) -> Result<HttpResponse, Error> {
    let mut player = player.lock().unwrap();

    player.resume();

    Ok(HttpResponse::Ok().finish())
}

#[post("pause")]
pub async fn pause(player: web::Data<PlayerData>) -> Result<HttpResponse, Error> {
    let mut player = player.lock().unwrap();

    player.pause();

    Ok(HttpResponse::Ok().finish())
}

#[post("stop")]
pub async fn stop(player: web::Data<PlayerData>) -> Result<HttpResponse, Error> {
    let mut player = player.lock().unwrap();

    player.stop().map_err(|err| {
        error!("{}", err);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().finish())
}

#[post("next")]
pub async fn next(player: web::Data<PlayerData>) -> Result<HttpResponse, Error> {
    let mut player = player.lock().unwrap();

    player.next().map_err(|err| {
        error!("{}", err);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().finish())
}

#[post("prev")]
pub async fn prev(player: web::Data<PlayerData>) -> Result<HttpResponse, Error> {
    let mut player = player.lock().unwrap();

    player.prev().map_err(|err| {
        error!("{}", err);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().finish())
}

#[post("seek")]
pub async fn seek(
    seek_to: web::Json<SeekTo>,
    player: web::Data<PlayerData>,
) -> Result<HttpResponse, Error> {
    let mut player = player.lock().unwrap();
    let seek_to = seek_to.into_inner();

    player
        .seek(Duration::from_millis(seek_to.seek_to))
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().finish())
}

#[post("volume")]
pub async fn set_volume(
    volume: web::Json<Volume>,
    player: web::Data<PlayerData>,
) -> Result<HttpResponse, Error> {
    let mut player = player.lock().unwrap();
    let volume = volume.into_inner();

    player.set_volume(volume.volume);

    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
pub struct Volume {
    volume: f32,
}

#[derive(Deserialize)]
pub struct SeekTo {
    seek_to: u64,
}
