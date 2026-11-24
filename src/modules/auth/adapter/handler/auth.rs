use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::modules::auth::{domain::entity::Credential, port::inbound::AuthService};

#[derive(Debug, Deserialize)]
pub struct SignInReqDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SignInResDto {
    pub code: String,
    pub message: String,
    pub data: SignInResDataDto,
}

#[derive(Debug, Serialize)]
pub struct SignInResDataDto {
    pub token: String,
}

pub async fn sign_in(
    State(service): State<Arc<dyn AuthService>>,
    Json(payload): Json<SignInReqDto>,
) -> Result<Json<SignInResDto>, StatusCode> {
    let c = Credential {
        username: payload.username,
        password: payload.password,
    };

    let create_token = service
        .sign_in(c)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let token = SignInResDataDto {
        token: create_token,
    };

    Ok(Json(SignInResDto {
        code: String::from("00"),
        message: String::from("authentication succeed"),
        data: token,
    }))
}
