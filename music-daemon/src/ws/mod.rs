use std::time::{Duration, Instant};

use actix::{ActorContext, ActorFuture, ContextFutureSpawner, fut, WrapFuture};
use actix::{Actor, Addr, Running, StreamHandler};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};
use uuid::Uuid;

use crate::ws::hub::WsHub;
use crate::ws::messages::{Connect, Disconnect, RodioCommandMessage, WsRodioStateMessage};

pub mod start_connection;
pub mod messages;
pub mod hub;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct ClientControllerWs {
    id: Uuid,
    hub_addr: Addr<WsHub>,
    hb: Instant,
}


impl ClientControllerWs {
    pub fn new(hub_addr: Addr<WsHub>) -> Self {
        Self {
            id: Uuid::new_v4(),
            hub_addr,
            hb: Instant::now(),
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                warn!("Websocket client heartbeat failed, disconnecting socket");

                act.hub_addr.do_send(Disconnect {
                    id: act.id
                });

                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for ClientControllerWs {
    type Context = ws::WebsocketContext<Self>;

    // Called on actor start
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.hub_addr.send(Connect {
            self_id: self.id,
            addr: addr.recipient(),
        })
            .into_actor(self)
            .then(|res, _, ctx| {
                if res.is_err() {
                    ctx.stop();
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        self.hub_addr.do_send(Disconnect {
            id: self.id
        });

        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ClientControllerWs {
    fn handle(
        &mut self,
        msg: Result<Message, ProtocolError>,
        ctx: &mut Self::Context) {
        match msg {
            // Client pings server
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            // Client sends answer to sent ping
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            // No operation
            Ok(ws::Message::Nop) => (),
            // Client sends text
            Ok(ws::Message::Text(text)) => {
                // TODO: Forward Message to hub
                let command = serde_json::from_str::<RodioCommandMessage>(&text);

                match command {
                    Ok(cmd) => {
                        self.hub_addr.do_send(cmd)
                    }
                    Err(err) => {
                        // Command could not be parsed
                        ctx.text(format!("Error parsing command: {}", err));
                    }
                }

                // Is a player command (play, next, ...)
            }
            // Client sends binary data
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl Handler<WsRodioStateMessage> for ClientControllerWs {
    type Result = ();

    fn handle(&mut self, msg: WsRodioStateMessage, ctx: &mut Self::Context) -> Self::Result {
        let json_string = serde_json::to_string(&msg.0).unwrap();
        ctx.text(json_string);
    }
}
