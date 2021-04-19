use actix::prelude::Message;
use actix::Recipient;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::player::RodioPlayerState;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsRodioStateMessage(pub RodioPlayerState);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub self_id: Uuid,
    pub addr: Recipient<WsRodioStateMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RodioCommand {
    Resume,
    Pause,
    Stop,
    Next,
    Prev,
    Seek(u64),
    Volume(f32),
    State,
}

#[derive(Debug, Message, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct RodioCommandMessage {
    pub command: RodioCommand,
}
