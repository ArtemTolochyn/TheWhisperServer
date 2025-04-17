use actix_web::{post, web, HttpResponse, Responder};
use crate::AppState;
use crate::models::{CreateChannelRequest, CreateChannelResponse, GetChannelListResponse, JoinChannelRequest, LeaveChannelRequest, RemoveChannelRequest};
use crate::services::auth_service::AuthenticatedUser;
use crate::services::channels_service;

#[post("/create_channel")]
pub async fn create_channel(data: web::Data<AppState>, req: web::Json<CreateChannelRequest>, _user: web::ReqData<AuthenticatedUser>,) -> impl Responder {
    let chat_name = req.name.clone();

    match channels_service::create_channel(&data.database, &chat_name).await {
        Ok(chat) => HttpResponse::Ok().json(CreateChannelResponse{ channel: chat}),
        Err(e) => e
    }
}

#[post("/join_channel")]
pub async fn join_channel(data: web::Data<AppState>, req: web::Json<JoinChannelRequest>, user: web::ReqData<AuthenticatedUser>,) -> impl Responder {
    let chat_id = req.id.clone();
    let user_id = user.user.id.clone();

    let key = req.key.clone();
    let signature = req.signature.clone();

    match channels_service::join_channel(&data.database, chat_id, user_id, &key, &signature).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e
    }
}


#[post("/remove_channel")]
pub async fn remove_channel(data: web::Data<AppState>, req: web::Json<RemoveChannelRequest>, user: web::ReqData<AuthenticatedUser>,) -> impl Responder {
    let chat_id = req.chat_id.clone();
    let user_id = user.user.id.clone();

    match channels_service::remove_channel(&data.database, chat_id, user_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e
    }
}

#[post("/leave_channel")]
pub async fn leave_channel(data: web::Data<AppState>, req: web::Json<LeaveChannelRequest>, user: web::ReqData<AuthenticatedUser>,) -> impl Responder {
    let chat_id = req.chat_id.clone();
    let user_id = user.user.id.clone();

    match channels_service::leave_channel(&data.database, chat_id, user_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e
    }
}

#[post("/get_channel_list")]
pub async fn get_channel_list(data: web::Data<AppState>, user: web::ReqData<AuthenticatedUser>,) -> impl Responder {
    let user_id = user.user.id.clone();

    match channels_service::get_channel_list(&data.database, user_id).await {
        Ok(channels) => HttpResponse::Ok().json(GetChannelListResponse{channels}),
        Err(e) => e
    }
}