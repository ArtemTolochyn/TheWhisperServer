use actix_web::{post, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::{AppState};
use crate::database::UserOpResult;
use crate::models::{ChallengeResponse, LoginRequest, LoginVerify, RegisterRequest, TokenResponse};
use crate::services::auth_service;
use crate::utils::crypto_utility;

#[post("/register")]
pub async fn register_user(data: web::Data<AppState>, req: web::Json<RegisterRequest>) -> impl Responder {
    let public_key = match auth_service::validate_public_key(&req.public_key) {
        Ok(key) => key,
        Err(e) => return e,
    };

    match auth_service::register_user(&data.database, &req.username, &public_key).await {
        UserOpResult::Ok(()) => HttpResponse::Ok().finish(),
        UserOpResult::AlreadyExists => HttpResponse::Conflict().body("Username already exists"),
        UserOpResult::InvalidInput => HttpResponse::BadRequest().body("Invalid input"),
        _ => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

#[post("/login/request")]
pub async fn login_request(data: web::Data<AppState>, req: web::Json<LoginRequest>) -> impl Responder {
    let challenge = Uuid::new_v4().to_string();

    let challenge_encrypted = match crypto_utility::encode_by_username(&data.database, &req.username, &challenge).await {
        Ok(e) => e,
        Err(e) => return e,
    };

    match auth_service::add_login_request(data.challenges.clone(), &req.username, &challenge, &challenge_encrypted) {
        Ok(challenge) => HttpResponse::Ok().json(ChallengeResponse{challenge}),
        Err(e) => e,
    }
}

#[post("/login/verify")]
pub async fn login_verify(data: web::Data<AppState>, req: web::Json<LoginVerify>) -> impl Responder {
    let challenge = match auth_service::get_challenge(data.challenges.clone(), &req.username) {
        Ok(c) => c,
        Err(e) => return e,
    };

    if req.response != challenge {
        return HttpResponse::Unauthorized().body("Wrong Private Key")
    }

    let token = match auth_service::add_session(&data.database, data.sessions.clone(), &req.username).await {
        Ok(t) => t,
        Err(e) => return e,
    };

    HttpResponse::Ok().json(TokenResponse { token })
}