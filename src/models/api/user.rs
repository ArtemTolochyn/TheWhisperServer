use serde::{Deserialize, Serialize};
use crate::models::database::channel::Channel;

//api/user/channels
#[derive(Serialize, Deserialize)]
pub struct ChannelsResponse
{
    pub channels: Vec<Channel>,
}
