use axum::{Json, Router, extract::State, routing::post};
use recipio_services::{RegisterUserDto, UserResponseDto};

use crate::{AppState, error::AppError};

/// Creates the router in charge of handling user level requests
pub fn user_router(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_user))
        .with_state(state)
}

/// Handles an HTTP request for creating a new user.
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserDto>,
) -> Result<Json<UserResponseDto>, AppError> {
    let user = state.users_service.register(payload).await?;
    Ok(Json(user))
}
