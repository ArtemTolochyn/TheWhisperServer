use actix_web::{post, web, HttpResponse, Responder};
use crate::AppState;
use crate::models::{GetMessagesRequest, GetMessagesResponse, RemoveMessageRequest, SendMessageRequest, SendMessageResponse};
use crate::services::auth_service::AuthenticatedUser;
use crate::services::messages_service;

#[post("/send_message")]
pub async fn send_message(data: web::Data<AppState>, req: web::Json<SendMessageRequest>, user: web::ReqData<AuthenticatedUser>,) -> impl Responder {
    let user_id = user.user.id.clone();
    let chat_id = req.chat_id.clone();

    let content = req.content.clone();

    match messages_service::send_message(&data.database, user_id, chat_id, &content).await
    {
        Ok(id) => HttpResponse::Ok().json(SendMessageResponse{message_id: id}),
        Err(e) => e
    }
}

#[post("/get_messages")]
pub async fn get_messages(data: web::Data<AppState>, req: web::Json<GetMessagesRequest>, user: web::ReqData<AuthenticatedUser>,) -> impl Responder {
    let chat_id = req.chat_id.clone();
    let user_id = user.user.id.clone();

    let message_id = req.message_id.clone();

    match messages_service::get_messages(&data.database, chat_id, user_id, message_id).await {
        Ok(messages) => HttpResponse::Ok().json(GetMessagesResponse{messages}),
        Err(e) => e
    }
}

#[post("/remove_message")]
pub async fn remove_message(data: web::Data<AppState>, req: web::Json<RemoveMessageRequest>, user: web::ReqData<AuthenticatedUser>) -> impl Responder {
    let message_id = req.message_id.clone();
    let user_id = user.user.id.clone();

    match messages_service::remove_message(&data.database, message_id, user_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e
    }
}