use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use actix_web::{web, Error, HttpMessage, HttpResponse};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use base64::Engine;
use rsa::RsaPublicKey;
use base64::engine::general_purpose::STANDARD as BASE64;
use rsa::pkcs8::DecodePublicKey;
use rsa::traits::PublicKeyParts;
use uuid::Uuid;
use crate::database::{Database, DatabaseGeneralError};
use crate::models::api::state::Session;
use crate::models::database::user::*;

#[derive(Clone)]
pub struct AuthenticatedUser {
    pub user: User,
}

impl AuthenticatedUser {
    pub fn new(user: User) -> Self {
        AuthenticatedUser { user }
    }
}

pub async fn auth_middleware(req: ServiceRequest, next: Next<impl MessageBody>, ) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let app_state = match req.app_data::<web::Data<crate::AppState>>() {
        Some(state) => state,
        None => return Err(actix_web::error::ErrorInternalServerError("Internal server error")),
    };

    let auth_header = match req.headers().get("Authorization") {
        Some(header) => header,
        None => return Err(actix_web::error::ErrorUnauthorized("Missing Authorization header")),
    };

    let token = match auth_header.to_str() {
        Ok(token) => token.trim(),
        Err(_) => return Err(actix_web::error::ErrorUnauthorized("Invalid token format")),
    };

    let sessions = match app_state.sessions.lock() {
        Ok(guard) => guard,
        Err(_) => return Err(actix_web::error::ErrorInternalServerError("Internal server error"))
    };

    let user_id = sessions
        .iter()
        .find(|(_key, value)| value.check_token(token.to_string()))
        .map(|(_key, value)| value.id)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    drop(sessions);

    match app_state.database.get_user_by_id(user_id).await {
        Ok(user) => {
            req.extensions_mut().insert(AuthenticatedUser::new(user));
        }
        Err(e) => {
            return match e {
                DatabaseGeneralError::NotFound => {
                    Err(actix_web::error::ErrorUnauthorized("Invalid token"))
                }
                _ => {
                    Err(actix_web::error::ErrorInternalServerError("Internal server error"))
                }
            }
        }
    };

    let res = next.call(req).await?;
    Ok(res)
}

pub fn validate_public_key(public_key: &str) -> Result<String, HttpResponse>
{
    let decoded_b64 = match BASE64.decode(public_key) {
        Ok(bytes) => bytes,
        Err(_) => return Err(HttpResponse::BadRequest().body("Invalid Base64 encoding")),
    };

    let pem_str = match String::from_utf8(decoded_b64) {
        Ok(s) => s,
        Err(_) => return Err(HttpResponse::BadRequest().body("Decoded data is not valid UTF-8")),
    };
    let public_key = match RsaPublicKey::from_public_key_pem(&pem_str) {
        Ok(pk) => pk,
        Err(_) => return Err(HttpResponse::BadRequest().body("Invalid public key format")),
    };

    if public_key.size() * 8 != 2048 {
        return Err(HttpResponse::BadRequest().body("Public key must be 2048 bits"));
    }

    Ok(pem_str)
}

pub async fn register_user(database: &Database, username: &str, public_key: &str) -> Result<(), DatabaseGeneralError>
{
    database.register_user(username, public_key).await
}



pub fn lock_challenges_map(challenges: &Arc<Mutex<HashMap<String, String>>>) -> Result<MutexGuard<HashMap<String, String>>, HttpResponse> {
    challenges.lock().map_err(|e| {
        eprintln!("Failed to lock challenge map: {}", e);
        HttpResponse::InternalServerError().body("Internal server error")
    })
}
pub fn add_login_request(challenges: Arc<Mutex<HashMap<String, String>>>, username: &str, challenge: &str, challenge_encrypted: &str) -> Result<String, HttpResponse>
{
    let mut map = match lock_challenges_map(&challenges) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };
    map.insert(username.to_string(), challenge.to_string());

    Ok(challenge_encrypted.to_string())
}


pub fn get_challenge(challenges: Arc<Mutex<HashMap<String, String>>>, username: &str) -> Result<String, HttpResponse>
{
    let map = match lock_challenges_map(&challenges) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let result = match map.get(username) {
        Some(c) => c.clone(),
        None => return Err(HttpResponse::Unauthorized().body("There is not request to login")),
    };
    Ok(result)
}

pub async fn add_session(database: &Database, sessions: Arc<Mutex<HashMap<String, Session>>>, username: &str) -> Result<String, HttpResponse>
{
    let token = Uuid::new_v4().to_string();

    let user = database.get_user_by_username(username).await
        .map_err(|_| HttpResponse::InternalServerError().body("Failed to get user ID"))?;

    let user_id = user.id;

    let mut sessions = match sessions.lock() {
        Ok(s) => s,
        Err(_) => return Err(HttpResponse::InternalServerError().body("Internal server error"))
    };

    match sessions.get(&user_id.to_string()) {
        Some(session) => {
            let mut new_user_sessions = session.clone();
            new_user_sessions.add_token(token.clone());
            sessions.insert(user_id.to_string(), new_user_sessions);
        },
        None => {
            let new_user_sessions = Session::new(user_id, token.clone());
            sessions.insert(user_id.to_string(), new_user_sessions);
        }
    };

    Ok(token)
}