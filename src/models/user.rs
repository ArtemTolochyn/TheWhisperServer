use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub(crate) username: String,
    pub(crate) public_key: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub(crate) username: String,
}

#[derive(Deserialize)]
pub struct LoginVerify {
    pub(crate) username: String,
    pub(crate) response: String,
}

#[derive(Serialize)]
pub struct ChallengeResponse {
    pub(crate) challenge: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub(crate) token: String,
}