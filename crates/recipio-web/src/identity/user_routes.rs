use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use recipio_core::{
    Id,
    identity::user::{Email, UnhashedPassword, User, Username},
};
use recipio_services::{UserResponseDto, UserService};
use serde::Deserialize;

use crate::{
    AppState,
    error::AppError,
    identity::auth::{AuthedUser, GuestUser},
    response::{Created, Success},
};

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Creates the router in charge of handling user level requests
pub fn user_router(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/{user_id}", get(get_user))
        .with_state(state)
}

/// Handles an HTTP request for creating a new user.
async fn create_user(
    State(users_service): State<UserService>,
    _guest: GuestUser,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Created<UserResponseDto>, AppError> {
    let username: Username = payload.username.try_into()?;
    let email: Email = payload.email.try_into()?;
    let password: UnhashedPassword = payload.password.try_into()?;

    let user = users_service.register(username, email, password).await?;
    Ok(Created(user))
}

async fn get_user(
    State(users_service): State<UserService>,
    Path(user_id): Path<Id<User>>,
    AuthedUser(requester): AuthedUser,
) -> Result<Success<Option<UserResponseDto>>, AppError> {
    let user = users_service.get_by_id(&user_id, &requester).await?;
    Ok(Success(user))
}
