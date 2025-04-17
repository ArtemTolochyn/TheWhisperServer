use actix_web::HttpResponse;
use crate::database::{Database, Message, UserOpResult};

pub async fn send_message(database: &Database, user_id: i64, chat_id: i64, content: &str) -> Result<i64, HttpResponse>
{
    match database.add_message(chat_id, user_id, content).await
    {
        UserOpResult::Ok(id) => Ok(id),
        UserOpResult::NotFound => Err(HttpResponse::NotFound().body("Channel does not exist")),
        UserOpResult::InvalidInput => Err(HttpResponse::BadRequest().body("Invalid input")),
        _ => Err(HttpResponse::InternalServerError().body("Internal server error"))
    }
}

pub async fn get_messages(database: &Database, chat_id: i64, user_id: i64, message_id: Option<i64>) -> Result<Vec<Message>, HttpResponse>
{
    match database.get_messages(chat_id, user_id, message_id).await
    {
        UserOpResult::Ok(messages) => Ok(messages),
        UserOpResult::NotFound => Err(HttpResponse::NotFound().body("Channel does not exist")),
        _ => Err(HttpResponse::InternalServerError().body("Internal server error"))
    }
}

pub async fn remove_message(database: &Database, message_id: i64, user_id: i64) -> Result<(), HttpResponse> {
    match database.remove_message(message_id, user_id).await {
        UserOpResult::Ok(_) => Ok(()),
        UserOpResult::NotFound => Err(HttpResponse::NotFound().body("Message not found")),
        _ => Err(HttpResponse::InternalServerError().body("Internal server error"))
    }
}