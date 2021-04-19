use actix_files::NamedFile;
use actix_web::{Error, get, HttpResponse, web, HttpRequest};

use crate::DbPool;

mod actions;

#[get("/{image_id}")]
pub async fn get_image(
    web::Path(image_id): web::Path<i32>,
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool!");

    let image = web::block(move || actions::find_image_by_id(image_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    return if let Some(image) = image {
        NamedFile::open(image.path)?.into_response(&req)
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}


#[get("/album/{album_id}")]
pub async fn get_album_image(
    web::Path(album_id): web::Path<i32>,
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool!");

    let image = web::block(move || actions::find_album_image(album_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    return if let Some(image) = image {
        NamedFile::open(image.path)?.into_response(&req)
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[get("/album/{album_id}/id")]
pub async fn get_album_image_id(
    web::Path(album_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool!");

    let image = web::block(move || actions::find_album_image(album_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    return if let Some(image) = image {
        Ok(HttpResponse::Ok().json(image.id))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[get("/artist/{artist_id}")]
pub async fn get_artist_image(
    web::Path(artist_id): web::Path<i32>,
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool!");

    let image = web::block(move || actions::find_artist_image(artist_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    return if let Some(image) = image {
        NamedFile::open(image.path)?.into_response(&req)
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[get("/artist/{artist_id}/id+")]
pub async fn get_artist_image_id(
    web::Path(artist_id): web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool!");

    let image = web::block(move || actions::find_artist_image(artist_id, &conn))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })?;

    return if let Some(image) = image {
        Ok(HttpResponse::Ok().json(image.id))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
