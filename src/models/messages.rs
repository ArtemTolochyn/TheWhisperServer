use crate::database::Message;

#[derive(serde::Deserialize)]
pub struct SendMessageRequest
{
    pub(crate) chat_id: i64,
    pub(crate) content: String
}


#[derive(serde::Serialize)]
pub struct SendMessageResponse
{
    pub(crate) message_id: i64
}

#[derive(serde::Deserialize)]
pub struct GetMessagesRequest
{
    pub(crate) chat_id: i64,
    pub(crate) message_id: Option<i64>
}

#[derive(serde::Serialize)]
pub struct GetMessagesResponse
{
    pub(crate) messages: Vec<Message>
}

#[derive(serde::Deserialize)]
pub struct RemoveMessageRequest {
    pub(crate) message_id: i64,
}