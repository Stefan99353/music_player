use actix_web::{Scope, web};

mod actions;
mod controller;

pub fn register(scope: Scope) -> Scope {
    scope.service(web::scope("playlists")
        .service(controller::all_playlists)
        .service(controller::add_playlist)
        .service(controller::get_playlist)
        .service(controller::update_playlist)
        .service(controller::delete_playlist)
        .service(controller::all_tracks)
        .service(controller::add_track)
        .service(controller::delete_track)
    )
}
