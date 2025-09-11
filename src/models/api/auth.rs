use serde::{Deserialize, Serialize};


//api/auth/register
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub public_key: String,
}

//api/auth/login/request
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub challenge: String,
}


//api/auth/login/validate
#[derive(Deserialize)]
pub struct ValidateRequest {
    pub username: String,
    pub challenge: String
}

#[derive(Serialize)]
pub struct ValidateResponse {
    pub token: String,
}