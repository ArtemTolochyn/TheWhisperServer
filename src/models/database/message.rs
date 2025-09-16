use serde::{Deserialize, Serialize};
use sqlx::FromRow;

//messages
#[derive(Serialize, Deserialize, FromRow)]
pub struct Message {
    id: i64,
    chat_id: i64,
    user_id: i64,
    content: String,
    timestamp: i64,
}