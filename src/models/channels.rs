use crate::database::{Chat, ChatInfo};

#[derive(serde::Deserialize)]
pub struct CreateChannelRequest
{
    pub(crate) name: String,
}

#[derive(serde::Serialize)]
pub struct CreateChannelResponse
{
    pub(crate) channel: ChatInfo,
}

#[derive(serde::Deserialize)]
pub struct JoinChannelRequest
{
    pub(crate) id: i64,
    pub(crate) key: String,
    pub(crate) signature: String,
}

#[derive(serde::Deserialize)]
pub struct RemoveChannelRequest {
    pub(crate) chat_id: i64,
}

#[derive(serde::Deserialize)]
pub struct LeaveChannelRequest {
    pub(crate) chat_id: i64,
}

#[derive(serde::Deserialize)]
pub struct GetChannelListRequest
{}


#[derive(serde::Serialize)]
pub struct GetChannelListResponse
{
    pub(crate) channels: Vec<Chat>,
}