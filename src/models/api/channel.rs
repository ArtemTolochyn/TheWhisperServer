use serde::{Serialize};

//api/channel/create
#[derive(Serialize)]
pub struct CreateRequest {
    pub name: String,
}

//api/channel/join
#[derive(Serialize)]
pub struct JoinRequest {
    pub id: String,
    pub key: String,
    pub signature: String,
}

//api/channel/remove
#[derive(Serialize)]
pub struct RemoveRequest {
    pub chat_id: i64,
}

//api/channel/leave
#[derive(Serialize)]
pub struct LeaveRequest {
    pub chat_id: i64,
}