use actix_web::HttpResponse;
use crate::database::{Chat, ChatInfo, Database, UserOpResult};

pub async fn create_channel(database: &Database, name: &str) -> Result<ChatInfo, HttpResponse>
{
    match database.create_chat(name).await {
        UserOpResult::Ok(chat) => Ok(chat),
        UserOpResult::InvalidInput => Err(HttpResponse::BadRequest().body("Invalid input")),
        _ => Err(HttpResponse::InternalServerError().body("Internal server error"))
    }
}

pub async fn join_channel(database: &Database, chat_id: i64, user_id: i64, key: &str, signature: &str) -> Result<(), HttpResponse>
{
    match database.add_user_to_chat(chat_id, user_id, key, signature).await {
        UserOpResult::Ok(id) => Ok(id),
        UserOpResult::NotFound => Err(HttpResponse::NotFound().body("Channel does not exist")),
        UserOpResult::InvalidInput => Err(HttpResponse::BadRequest().body("Invalid input")),
        UserOpResult::AlreadyExists => Err(HttpResponse::Conflict().body("User already in the chat")),
        _ => Err(HttpResponse::InternalServerError().body("Internal server error"))
    }
}

pub async fn leave_channel(database: &Database, chat_id: i64, user_id: i64) -> Result<(), HttpResponse>
{
    match database.remove_user_from_chat(chat_id, user_id).await {
        UserOpResult::Ok(id) => Ok(id),
        UserOpResult::NotFound => Err(HttpResponse::NotFound().body("Channel does not exist")),
        _ => Err(HttpResponse::InternalServerError().body("Internal server error"))
    }
}




pub async fn remove_channel(database: &Database, chat_id: i64, user_id: i64) -> Result<(), HttpResponse>
{
    match database.remove_chat(chat_id, user_id).await {
        UserOpResult::Ok(id) => Ok(id),
        UserOpResult::NotFound => Err(HttpResponse::NotFound().body("Channel does not exist")),
        _ => Err(HttpResponse::InternalServerError().body("Internal server error"))
    }
}


pub async fn get_channel_list(database: &Database, user_id: i64) -> Result<Vec<Chat>, HttpResponse>
{
    match database.get_chats_by_user_id(user_id).await {
        UserOpResult::Ok(id) => Ok(id),
        _ => Err(HttpResponse::InternalServerError().body("Internal server error"))
    }
}