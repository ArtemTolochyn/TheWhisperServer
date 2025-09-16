use serde::{Deserialize, Serialize};
use crate::models::database::message::Message;

//api/message/send
#[derive(Serialize, Deserialize)]
pub struct SendRequest {
    pub chat_id: i64,
    pub content: String,
}

//api/message/chat
#[derive(Serialize, Deserialize)]
pub struct ChatRequest {
    pub chat_id: i64,
    pub message_id: Option<i64>
}

#[derive(Serialize, Deserialize)]
pub struct ChatResponse
{
    pub messages: Vec<Message>
}

//api/message/remove
#[derive(Serialize, Deserialize)]
pub struct RemoveRequest {
    pub message_id: i64
}