use serde::{Deserialize, Serialize};


//messages
#[derive(Serialize, Deserialize)]
pub struct Message {
    id: i64,
    chat_id: i64,
    user_id: i64,
    content: String,
    timestamp: i64,
}