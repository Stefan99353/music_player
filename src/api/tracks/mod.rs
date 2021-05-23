use actix_web::{Scope, web};

mod actions;
mod controller;

pub fn register(scope: Scope) -> Scope {
    scope.service(web::scope("tracks")
        .service(controller::all_tracks)
        .service(controller::add_track)
        .service(controller::get_track)
        .service(controller::update_track)
        .service(controller::delete_track)
        .service(controller::stream_track)
        .service(controller::add_image)
        .service(controller::delete_image)
    )
}
