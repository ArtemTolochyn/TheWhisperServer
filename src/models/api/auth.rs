use serde::{Deserialize, Serialize};
use serde_valid::Validate;

//api/auth/register
#[derive(Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(min_length = 3)]
    #[validate(max_length = 20)]
    pub username: String,
    pub public_key: String,
}

//api/auth/login/request
#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub challenge: String,
}


//api/auth/login/validate
#[derive(Serialize, Deserialize)]
pub struct ValidateRequest {
    pub username: String,
    pub challenge: String
}

#[derive(Serialize, Deserialize)]
pub struct ValidateResponse {
    pub token: String,
}
