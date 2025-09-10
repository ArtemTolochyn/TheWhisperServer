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

}