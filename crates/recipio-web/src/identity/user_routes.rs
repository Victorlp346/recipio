use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use recipio_core::{Id, identity::user::User};
use recipio_services::{RegisterUserDto, UserResponseDto};

use crate::{
    AppState,
    error::AppError,
    identity::auth::{AuthedUser, GuestUser},
    response::{Created, Success},
};

/// Creates the router in charge of handling user level requests
pub fn user_router(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/{user_id}", get(get_user))
        .with_state(state)
}

/// Handles an HTTP request for creating a new user.
async fn create_user(
    State(state): State<AppState>,
    _guest: GuestUser,
    Json(payload): Json<RegisterUserDto>,
) -> Result<Created<UserResponseDto>, AppError> {
    let user = state.users_service.register(payload).await?;
    Ok(Created(user))
}

async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Id<User>>,
    AuthedUser(requester): AuthedUser,
) -> Result<Success<Option<UserResponseDto>>, AppError> {
    let user = state.users_service.get_by_id(&user_id, &requester).await?;
    Ok(Success(user))
}
