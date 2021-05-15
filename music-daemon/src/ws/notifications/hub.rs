use std::collections::HashMap;

use actix::{Actor, Context, Handler, Recipient};
use uuid::Uuid;

use super::messages::{Connect, Disconnect, Notification, WsNotificationMessage};

type Socket = Recipient<WsNotificationMessage>;

pub struct WsNotificationHub {
    sessions: HashMap<Uuid, Socket>,
}

impl WsNotificationHub {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    fn send_notification_to(&self, msg: Notification, to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(to) {
            if let Err(err) = socket_recipient.do_send(WsNotificationMessage(msg)) {
                error!("Error while pushing player state to client {}", to);
                error!("{}", err);
            }
        } else {
            // Uuid is not in sessions
            warn!("Attempting to send state to unknown client");
        }
    }
}

impl Actor for WsNotificationHub {
    type Context = Context<Self>;
}

// Connect Message
impl Handler<Connect> for WsNotificationHub {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.insert(msg.self_id, msg.addr);
    }
}

// Disconnect Message
impl Handler<Disconnect> for WsNotificationHub {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<Notification> for WsNotificationHub {
    type Result = ();

    fn handle(&mut self, msg: Notification, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.iter().for_each(|(client, _)| {
            self.send_notification_to(msg.clone(), client)
        })
    }
}
