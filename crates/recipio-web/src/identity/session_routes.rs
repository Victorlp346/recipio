use axum::{Json, Router, extract::State, routing::post};
use recipio_services::{LoginDto, SessionCreatedDTO};

use crate::{AppState, error::AppError, response::Created};

pub fn session_routes(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_session))
        .with_state(state)
}

#[axum::debug_handler]
async fn create_session(
    State(state): State<AppState>,
    Json(payload): Json<LoginDto>,
) -> Result<Created<SessionCreatedDTO>, AppError> {
    Ok(Created(
        state.session_service.create_session(payload).await?,
    ))
}
