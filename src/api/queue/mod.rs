use actix_web::{Scope, web};

mod actions;
mod controller;

pub fn register(scope: Scope) -> Scope {
    scope.service(web::scope("queue")
        .service(controller::all_tracks)
        .service(controller::clear_queue)
        .service(controller::length)
        .service(controller::add_to_queue)
        .service(controller::add_artist_to_queue)
        .service(controller::add_album_to_queue)
    )
}
