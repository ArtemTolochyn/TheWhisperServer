use serde::{Deserialize, Serialize};
use sqlx::FromRow;

//users
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub public_key: String,
}