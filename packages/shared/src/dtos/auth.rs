use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterStepAccount {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterStepProfile {
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub display_name: String,
    pub bio: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub account: RegisterStepAccount,
    pub profile: RegisterStepProfile,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthResponse {
    pub token: String,
}
