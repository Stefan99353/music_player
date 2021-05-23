use actix::Addr;
use actix_web::{Error, get, HttpRequest, HttpResponse};
use actix_web::web::{Data, Payload};
use actix_web_actors::ws;

use super::hub::WsNotificationHub;
use super::WsNotificationClientController;

#[get("notifications")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    hub: Data<Addr<WsNotificationHub>>,
) -> Result<HttpResponse, Error> {
    let ws = WsNotificationClientController::new(
        hub.get_ref().clone(),
    );

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}
