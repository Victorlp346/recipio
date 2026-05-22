use axum::{Json, Router, extract::State, routing::post};
use recipio_core::{Id, session::Session};
use recipio_services::LoginDto;

use crate::{AppState, error::AppError};

pub fn session_routes(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_session))
        .with_state(state)
}

#[axum::debug_handler]
async fn create_session(
    State(state): State<AppState>,
    Json(payload): Json<LoginDto>,
) -> Result<Json<Id<Session>>, AppError> {
    Ok(Json(state.session_service.create_session(payload).await?))
}
