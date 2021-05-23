use actix_web::{Scope, web};

mod actions;
mod controller;

pub fn register(scope: Scope) -> Scope {
    scope.service(web::scope("management")
        .service(controller::update_db)
        .service(controller::rebuild_db)
        .service(controller::all_updates)
    )
}
