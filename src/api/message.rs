use actix_web::{post, web, HttpResponse, Responder};
use crate::AppState;
use crate::services::auth_service::AuthenticatedUser;
use crate::services::messages_service;
use crate::models::api::message::*;

#[post("/send")]
pub async fn send(data: web::Data<AppState>, req: web::Json<SendRequest>, user: web::ReqData<AuthenticatedUser>,) -> impl Responder {
    let user_id = user.user.id.clone();
    let chat_id = req.chat_id.clone();

    let content = req.content.clone();

    match messages_service::send_message(&data.database, user_id, chat_id, &content).await
    {
        Ok(id) => HttpResponse::Ok().finish(),
        Err(e) => e
    }
}

#[post("/remove")]
pub async fn remove(data: web::Data<AppState>, req: web::Json<RemoveRequest>, user: web::ReqData<AuthenticatedUser>) -> impl Responder {
    let message_id = req.message_id.clone();
    let user_id = user.user.id.clone();

    match messages_service::remove_message(&data.database, message_id, user_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e
    }
}

#[post("/chat")]
pub async fn chat(data: web::Data<AppState>, req: web::Json<ChatRequest>, user: web::ReqData<AuthenticatedUser>,) -> impl Responder {
    let chat_id = req.chat_id.clone();
    let user_id = user.user.id.clone();

    let message_id = req.message_id.clone();

    match messages_service::get_messages(&data.database, chat_id, user_id, message_id).await {
        Ok(messages) => HttpResponse::Ok().json(ChatResponse{messages}),
        Err(e) => e
    }
}