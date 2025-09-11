use serde::Serialize;

//api/message/send
#[derive(Serialize)]
pub struct SendRequest {
    pub chat_id: i64,
    pub content: String,
}

//api/message/chat
#[derive(Serialize)]
pub struct ChatRequest {
    pub chat_id: i64,
    pub message_id: Option<i64>
}

//api/message/remove
#[derive(Serialize)]
pub struct RemoveRequest {
    pub message_id: i64
}