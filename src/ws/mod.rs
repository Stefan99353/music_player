use actix_web::{Scope, web};

pub mod player;
pub mod notifications;

pub fn register(scope: Scope) -> Scope {
    scope.service(web::scope("sockets")
        .service(player::start_connection::start_connection)
        .service(notifications::start_connection::start_connection)
    )
}
