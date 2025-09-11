use serde::Serialize;
use crate::database::Chat;

//api/user/channels
#[derive(Serialize)]
pub struct ChannelsRequest
{
    pub channels: Vec<Chat>,
}
