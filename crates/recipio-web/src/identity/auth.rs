use std::str::FromStr;

use axum::{
    extract::{FromRequestParts, Request, State},
    http::{HeaderMap, StatusCode, header::AUTHORIZATION, request::Parts},
    middleware::Next,
    response::Response,
};
use recipio_core::{
    Id, RecipioError,
    identity::auth::UserClaims,
    identity::session::Session,
    identity::user::{Role, User},
};
use recipio_services::{SessionService, UserService};

use crate::error::AppError;

const INVALID_SESSION_ERROR: AppError = AppError(RecipioError::Session(
    recipio_core::identity::session::SessionError::InvalidSession,
));

pub async fn retrieve_session_middleware(
    headers: HeaderMap,
    State(users_service): State<UserService>,
    State(session_service): State<SessionService>,
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

        let session = session_service
            .validate_session(&session_id, parts[1])
            .await
            .map_err(|_| INVALID_SESSION_ERROR)?;

        let Some(claims) = users_service.get_claims_by_id(session.user_id()).await? else {
            return Err(INVALID_SESSION_ERROR);
        };

        request.extensions_mut().insert(session);
        request.extensions_mut().insert(claims);
    }

    let response = next.run(request).await;
    Ok(response)
}

pub struct MaybeAuthenticated(pub Option<UserClaims>);

impl<S> FromRequestParts<S> for MaybeAuthenticated
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self(parts.extensions.get::<UserClaims>().cloned()))
    }
}

pub struct AuthedUser(pub UserClaims);

impl<S> FromRequestParts<S> for AuthedUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Some(user_claims) = parts.extensions.get::<UserClaims>().cloned() else {
            return Err(AppError(RecipioError::Unauthorized));
        };

        Ok(AuthedUser(user_claims))
    }
}

pub struct AdminUser(pub UserClaims);

impl<S> FromRequestParts<S> for AdminUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Some(user_claims) = parts.extensions.get::<UserClaims>().cloned() else {
            return Err(AppError(RecipioError::Unauthorized));
        };

        if !(*user_claims.role() >= Role::Admin) {
            return Err(AppError(RecipioError::Unauthorized));
        }

        Ok(Self(user_claims))
    }
}

pub struct GuestUser;

impl<S> FromRequestParts<S> for GuestUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(_user) = parts.extensions.get::<User>() {
            return Err(AppError(RecipioError::AlreadyAuthenticated));
        }
        Ok(Self)
    }
}
