use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix::{Actor, Context, Handler, Recipient};
use uuid::Uuid;

use crate::player::{RodioPlayer, RodioPlayerState};
use crate::ws::messages::{Connect, Disconnect, RodioCommand, RodioCommandMessage, WsRodioStateMessage};

type Socket = Recipient<WsRodioStateMessage>;

pub struct WsHub {
    player: Arc<Mutex<RodioPlayer>>,
    sessions: HashMap<Uuid, Socket>,
}

impl WsHub {
    pub fn new(player: Arc<Mutex<RodioPlayer>>) -> Self {
        Self {
            player,
            sessions: HashMap::new(),
        }
    }

    fn send_state_to(&self, msg: RodioPlayerState, to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(to) {
            if let Err(err) = socket_recipient.do_send(WsRodioStateMessage(msg)) {
                error!("Error while pushing player state to client {}", to);
                error!("{}", err);
            }
        } else {
            // Uuid is not in sessions
            warn!("Attempting to send state to unknown client");
        }
    }
}

impl Actor for WsHub {
    type Context = Context<Self>;
}

// Connect Message
impl Handler<Connect> for WsHub {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.insert(msg.self_id, msg.addr);

        let player = self.player.lock().unwrap();
        let state = player.get_state();
        drop(player);

        self.send_state_to(state, &msg.self_id);
    }
}

// Disconnect Message
impl Handler<Disconnect> for WsHub {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<RodioCommandMessage> for WsHub {
    type Result = ();

    fn handle(&mut self, msg: RodioCommandMessage, _ctx: &mut Self::Context) -> Self::Result {
        let mut player = self.player.lock().unwrap();

        // TODO: Handle commands
        match msg.command {
            RodioCommand::Resume => {
                player.resume();
            }
            RodioCommand::Pause => {
                player.pause();
            }
            RodioCommand::Stop => {
                let _ = player.stop();
            }
            RodioCommand::Next => {
                let _ = player.next();
            }
            RodioCommand::Prev => {
                let _ = player.prev();
            }
            RodioCommand::Seek(seek_to) => {
                let _ = player.seek(Duration::from_millis(seek_to));
            }
            RodioCommand::Volume(new_volume) => {
                player.set_volume(new_volume);
            }
            RodioCommand::State => {
                let state = player.get_state();
                drop(player);

                self.sessions.iter().for_each(|(client, _)| {
                    self.send_state_to(state.clone(), client);
                });
            }
        }
    }
}
