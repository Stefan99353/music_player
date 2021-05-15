use actix::Addr;
use actix_web::{Error, get, HttpRequest, HttpResponse};
use actix_web::web::{Data, Payload};
use actix_web_actors::ws;

use super::hub::WsPlayerHub;
use super::WsPlayerClientController;

#[get("player")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    hub: Data<Addr<WsPlayerHub>>,
) -> Result<HttpResponse, Error> {
    let ws = WsPlayerClientController::new(
        hub.get_ref().clone(),
    );

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}
