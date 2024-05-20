use axum::extract::{FromRequestParts, Query};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::{async_trait, RequestPartsExt};
use headers::authorization::Basic;
use serde::Deserialize;
use std::collections::HashMap;
use log::info;

pub struct RequireAuth;

pub enum AuthType {
    None,
    /// user, password
    Clear(String, String), // mostly obsolete
    // hash, salt
    Hashed(String, String),
}

#[async_trait]
impl FromRequestParts<()> for RequireAuth {
    type Rejection = axum::http::StatusCode;

    /// use state to propagate user credentials
    async fn from_request_parts(parts: &mut Parts, state: &()) -> Result<Self, Self::Rejection> {
        let query_params = parts
            .extract::<Query<HashMap<String, String>>>()
            .await
            .map(|Query(params)| params)
            .map_err(|err| err.into_response());

        match query_params {
            Ok(hashmap) => {
                let client_id=hashmap.get("c").unwrap_or(&"unknown".to_string()).clone();
                if check_user_auth(hashmap) {
                    info!("Subsonic client '{}' authorized", client_id);
                    return Ok(Self);
                } else {
                    info!("Subsonic client '{}' not authorized", client_id);
                    return Err(axum::http::StatusCode::UNAUTHORIZED);
                }
            }
            Err(_) => return Err(axum::http::StatusCode::UNAUTHORIZED),
        };
    }
}

fn select_auth_type(hashmap: HashMap<String, String>) -> AuthType {
    //TODO
    AuthType::None
}

/// returns true if the user is authorized
fn check_user_auth(hashmap: HashMap<String, String>) -> bool {
    match select_auth_type(hashmap) {
        AuthType::None => true,
        AuthType::Clear(user, password) => check_clear_password(&user, &password),
        AuthType::Hashed(hash, salt) => check_hash(&hash, &salt),
    }
}

fn check_clear_password(user: &str, password: &str) -> bool {
    todo!("implcleartextauth")
}
fn check_hash(hash: &str, salt: &str) -> bool {
    todo!("impl hash auth")
}
