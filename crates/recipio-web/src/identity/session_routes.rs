use axum::{Json, Router, extract::State, routing::post};
use recipio_core::identity::user::{UnhashedPassword, Username};
use recipio_services::SessionCreatedDTO;
use serde::Deserialize;

use crate::{AppState, error::AppError, response::Created};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub fn session_routes(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_session))
        .with_state(state)
}

#[axum::debug_handler]
async fn create_session(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Created<SessionCreatedDTO>, AppError> {
    let username: Username = payload.username.try_into()?;
    let password: UnhashedPassword = payload.password.try_into()?;

    Ok(Created(
        state
            .session_service
            .create_session(username, password)
            .await?,
    ))
}
