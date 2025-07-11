use serde::{Deserialize, Serialize};

use crate::models::_entities::users;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub name: String,
    pub is_verified: bool,
    pub is_admin: bool,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &users::Model, token: &String) -> Self {
        Self {
            token: token.to_string(),
            pid: String::from_utf8_lossy(&user.pid).to_string(),
            name: user.name.clone(),
            is_verified: user.email_verified_at.is_some(),
            is_admin: (user.is_admin.unwrap_or(0) != 0),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
    pub pid: String,
    pub name: String,
    pub email: String,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &users::Model) -> Self {
        Self {
            pid: String::from_utf8_lossy(&user.pid).to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
        }
    }
}
