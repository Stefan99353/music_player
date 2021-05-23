use actix_web::{Scope, web};

mod controller;

pub fn register(scope: Scope) -> Scope {
    scope.service(web::scope("player")
        .service(controller::state)
        .service(controller::resume)
        .service(controller::pause)
        .service(controller::stop)
        .service(controller::next)
        .service(controller::prev)
        .service(controller::seek)
        .service(controller::set_volume)
    )
}
