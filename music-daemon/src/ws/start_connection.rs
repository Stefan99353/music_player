use actix::Addr;
use actix_web::{Error, get, HttpRequest, HttpResponse, web::Data, web::Payload, Scope};
use actix_web_actors::ws;

use crate::ws::ClientControllerWs;
use crate::ws::hub::WsHub;

pub fn register(scope: Scope) -> Scope {
    scope.service(start_connection)
}

#[get("/ws")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    hub: Data<Addr<WsHub>>,
) -> Result<HttpResponse, Error> {
    let ws = ClientControllerWs::new(
        hub.get_ref().clone(),
    );

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}
