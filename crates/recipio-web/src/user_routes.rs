use axum::{Json, Router, routing::post};
use recipio_core::{CreateUserDTO, RecipioError};
use serde::Deserialize;

use crate::error::AppError;

pub fn user_router() -> Router {
    Router::new().route("/", post(create_user))
}

#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    password: String,
    email: String,
}

impl TryFrom<CreateUserRequest> for CreateUserDTO {
    type Error = RecipioError;

    fn try_from(req: CreateUserRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            username: req.username.try_into()?,
            email: req.email.try_into()?,
            password: req.password.try_into()?,
        })
    }
}

async fn create_user(Json(payload): Json<CreateUserRequest>) -> Result<String, AppError> {
    let create_user_data: CreateUserDTO = payload.try_into()?;
    dbg!(create_user_data);
    Ok("Hello, World".to_string())
}
