use serde::{Deserialize, Serialize};

//api/channel/create
#[derive(Serialize, Deserialize)]
pub struct CreateRequest {
    pub name: String,
}

//api/channel/join
#[derive(Serialize, Deserialize)]
pub struct JoinRequest {
    pub id: i64,
    pub key: String,
    pub signature: String,
}

//api/channel/remove
#[derive(Serialize, Deserialize)]
pub struct RemoveRequest {
    pub chat_id: i64,
}

//api/channel/leave
#[derive(Serialize, Deserialize)]
pub struct LeaveRequest {
    pub chat_id: i64,
}