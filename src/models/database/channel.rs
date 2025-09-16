use serde::{Deserialize, Serialize};
use sqlx::FromRow;

//channels
#[derive(Serialize, Deserialize, FromRow)]
pub struct Channel {
    pub id: i64,
    pub name: String,
}

//chat_users
#[derive(Serialize, Deserialize, FromRow)]
pub struct ChatUser {
    pub id: i64,
    pub chat_id: i64,
    pub user_id: i64,
    pub key: String,
    pub signature: String,
}