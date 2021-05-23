use actix_web::{Scope, web};

mod actions;
mod controller;

pub fn register(scope: Scope) -> Scope {
    scope.service(web::scope("albums")
        .service(controller::all_albums)
        .service(controller::add_album)
        .service(controller::get_album)
        .service(controller::update_album)
        .service(controller::delete_album)
        .service(controller::all_tracks)
        .service(controller::add_image)
        .service(controller::delete_image)
    )
}
