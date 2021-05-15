use actix::prelude::Message;
use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use actix::Recipient;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsNotificationMessage(pub Notification);

#[derive(Debug, Clone, Message, Serialize, Deserialize)]
#[rtype(result = "()")]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub message: String,
    pub message_type: NotificationType,
    pub timestamp: NaiveDateTime
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NotificationType {
    Default,
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub self_id: Uuid,
    pub addr: Recipient<WsNotificationMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}
