use actix_web::{Scope, web};

mod controller;

pub fn register(scope: Scope) -> Scope {
    scope.service(web::scope("images")
        .service(controller::get_image)
    )
}
