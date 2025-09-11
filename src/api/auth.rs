use actix_web::{post, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::{AppState};
use crate::database::{DatabaseGeneralError};
use crate::services::auth_service;
use crate::utils::crypto_utility;
use crate::models::api::auth::*;

#[post("/register")]
pub async fn register(data: web::Data<AppState>, req: web::Json<RegisterRequest>) -> impl Responder {
    let public_key = match auth_service::validate_public_key(&req.public_key) {
        Ok(key) => key,
        Err(e) => return e,
    };

    match auth_service::register_user(&data.database, &req.username, &public_key).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            match e {
                DatabaseGeneralError::AlreadyExists => HttpResponse::Conflict().body("Username already exists"),
                DatabaseGeneralError::InvalidInput => HttpResponse::BadRequest().body("Invalid input"),
                _ => HttpResponse::InternalServerError().body("Internal server error"),
            }
        }
    }
}

#[post("/request")]
pub async fn request(data: web::Data<AppState>, req: web::Json<LoginRequest>) -> impl Responder {
    let challenge = Uuid::new_v4().to_string();

    let challenge_encrypted = match crypto_utility::encode_by_username(&data.database, &req.username, &challenge).await {
        Ok(e) => e,
        Err(e) => return e,
    };

    match auth_service::add_login_request(data.challenges.clone(), &req.username, &challenge, &challenge_encrypted) {
        Ok(challenge) => HttpResponse::Ok().json(LoginResponse{challenge}),
        Err(e) => e,
    }
}

#[post("/validate")]
pub async fn validate(data: web::Data<AppState>, req: web::Json<ValidateRequest>) -> impl Responder {
    let challenge = match auth_service::get_challenge(data.challenges.clone(), &req.username) {
        Ok(c) => c,
        Err(e) => return e,
    };

    if req.challenge != challenge {
        return HttpResponse::Unauthorized().body("Wrong Private Key")
    }

    let token = match auth_service::add_session(&data.database, data.sessions.clone(), &req.username).await {
        Ok(t) => t,
        Err(e) => return e,
    };

    HttpResponse::Ok().json(ValidateResponse { token })
}