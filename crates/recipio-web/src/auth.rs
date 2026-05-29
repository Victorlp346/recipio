use std::str::FromStr;

use axum::{
    extract::{Request, State},
    http::{HeaderMap, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};
use recipio_core::{Id, RecipioError, session::Session};

use crate::{AppState, error::AppError};

const INVALID_SESSION_ERROR: AppError = AppError(RecipioError::Session(
    recipio_core::session::SessionError::InvalidSession,
));

pub async fn retrieve_session_middleware(
    headers: HeaderMap,
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    if let Some(auth_header) = headers.get(AUTHORIZATION) {
        let Some(token) = auth_header.to_str().ok() else {
            return Err(INVALID_SESSION_ERROR);
        };

        if !token.starts_with("Bearer ") {
            return Err(INVALID_SESSION_ERROR);
        }

        let token_str = &token["Bearer ".len()..];
        let parts: Vec<&str> = token_str.splitn(2, ':').collect();

        let session_id: Id<Session> = Id::from_str(parts[0])
            .map_err(|_| INVALID_SESSION_ERROR)?
            .into();

        let session = state
            .session_service
            .validate_session(&session_id, parts[1])
            .await
            .map_err(|_| INVALID_SESSION_ERROR)?;

        dbg!(session.clone());
        request.extensions_mut().insert(session);
    }

    let response = next.run(request).await;
    Ok(response)
}
