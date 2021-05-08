use actix_web::{Scope, web};

mod actions;
mod controller;

pub fn register(scope: Scope) -> Scope {
    scope.service(web::scope("artists")
        .service(controller::all_artists)
        .service(controller::add_artist)
        .service(controller::get_artist)
        .service(controller::update_artist)
        .service(controller::delete_artist)
        .service(controller::all_albums)
        .service(controller::all_tracks)
        .service(controller::add_image)
        .service(controller::delete_image)
    )
}
