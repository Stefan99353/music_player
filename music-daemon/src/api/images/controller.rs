use actix_web::{web, HttpRequest, HttpResponse, Error, get};
use crate::DbPool;
use crate::models::images::Image;
use actix_files::NamedFile;

#[get("/{image_id}")]
pub async fn get_image(
    web::Path(image_id): web::Path<i32>,
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool!");

    let image = web::block(move || Image::get(image_id, &conn))
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
