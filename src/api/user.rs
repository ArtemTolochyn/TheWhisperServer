use actix_web::{post, web, HttpResponse, Responder};
use crate::AppState;
use crate::models::api::user::ChannelsResponse;
use crate::services::auth_service::AuthenticatedUser;
use crate::services::channels_service;

#[post("/channels")]
pub async fn channels(data: web::Data<AppState>, user: web::ReqData<AuthenticatedUser>) -> impl Responder {
    let user_id = user.user.id.clone();

    match channels_service::get_channel_list(&data.database, user_id).await {
        Ok(channels) => HttpResponse::Ok().json(ChannelsResponse{channels}),
        Err(e) => e
    }
}