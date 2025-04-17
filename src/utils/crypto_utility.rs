use actix_web::HttpResponse;
use base64::Engine;
use base64::engine::general_purpose;
use rand::thread_rng;
use rsa::{Oaep, RsaPublicKey};
use rsa::pkcs8::DecodePublicKey;
use sha2::Sha256;
use crate::database::{Database, UserOpResult};

pub async fn encode_by_username(database: &Database, username: &str, challenge: &str) -> Result<String, HttpResponse>
{
    let public_key_str = match database.get_user_public_key(&username).await {
        UserOpResult::Ok(key) => key,
        _ => return Err(HttpResponse::NotFound().body("User not found")),
    };

    let public_key = match RsaPublicKey::from_public_key_pem(&public_key_str) {
        Ok(pk) => pk,
        Err(e) => {
            eprintln!("Failed to parse public key DER: {}", e);
            return Err(HttpResponse::InternalServerError().body("Invalid public key"));
        }
    };

    let mut rng = thread_rng();
    let padding = Oaep::new::<Sha256>();
    let encrypted = match public_key.encrypt(&mut rng, padding, challenge.as_bytes()) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Encryption error: {}", e);
            return Err(HttpResponse::InternalServerError().body("Encryption failed"));
        }
    };

    Ok(general_purpose::STANDARD.encode(encrypted))
}