use actix_web::{post, web, HttpResponse, Responder};
use crate::AppState;
use crate::services::auth_service::AuthenticatedUser;
use crate::services::channels_service;
use crate::models::api::channel::*;

#[post("/create")]
pub async fn create(data: web::Data<AppState>, req: web::Json<CreateRequest>, _user: web::ReqData<AuthenticatedUser>,) -> impl Responder {
    let chat_name = req.name.clone();

    match channels_service::create_channel(&data.database, &chat_name).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e
    }
}

#[post("/join")]
pub async fn join(data: web::Data<AppState>, req: web::Json<JoinRequest>, user: web::ReqData<AuthenticatedUser>,) -> impl Responder {
    let chat_id = req.id.clone();
    let user_id = user.user.id.clone();

    let key = req.key.clone();
    let signature = req.signature.clone();

    match channels_service::join_channel(&data.database, chat_id, user_id, &key, &signature).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e
    }
}


#[post("/remove")]
pub async fn remove(data: web::Data<AppState>, req: web::Json<RemoveRequest>, user: web::ReqData<AuthenticatedUser>,) -> impl Responder {
    let chat_id = req.chat_id.clone();
    let user_id = user.user.id.clone();

    match channels_service::remove_channel(&data.database, chat_id, user_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e
    }
}

#[post("/leave")]
pub async fn leave(data: web::Data<AppState>, req: web::Json<LeaveRequest>, user: web::ReqData<AuthenticatedUser>,) -> impl Responder {
    let chat_id = req.chat_id.clone();
    let user_id = user.user.id.clone();

    match channels_service::leave_channel(&data.database, chat_id, user_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e
    }
}