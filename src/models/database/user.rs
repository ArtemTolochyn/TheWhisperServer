use serde::{Deserialize, Serialize};

//users
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub public_key: String,
}