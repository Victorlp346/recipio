use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use recipio_core::identity::user::{EmailError, UnhashedPasswordError, UserError, UsernameError};
use recipio_core::{RecipioError, identity::session::SessionError};
use serde_json::json;

pub struct AppError(pub RecipioError);

impl From<RecipioError> for AppError {
    fn from(err: RecipioError) -> Self {
        AppError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self.0 {
            RecipioError::Unauthorized => (StatusCode::UNAUTHORIZED, "not authorized".to_string()),
            RecipioError::AlreadyAuthenticated => (
                StatusCode::BAD_REQUEST,
                "user is already authenticated".to_string(),
            ),
            RecipioError::User(user_error) => map_user_error(user_error),
            RecipioError::Session(session_error) => map_session_error(session_error),
            RecipioError::ParsingError { value: _, target } => (
                StatusCode::BAD_REQUEST,
                format!("Invalid format: expected {target}"),
            ),

            RecipioError::Repo(repo_err) => {
                tracing::error!("Database error: {:?}", repo_err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred".to_string(),
                )
            }

            RecipioError::HashingError => {
                tracing::error!("Password hashing error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred".to_string(),
                )
            }

            RecipioError::Unknown => {
                tracing::error!("Unknown app error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

fn map_user_error(err: &UserError) -> (StatusCode, String) {
    let msg = match err {
        UserError::InvalidEmail(e) => match e {
            EmailError::LenCharMaxViolated => {
                "Email address is too long. Please use a shorter email."
            }
            EmailError::RegexViolated => {
                "Please provide a valid email address (e.g., you@example.com)."
            }
        },
        UserError::InvalidUsername(e) => match e {
            UsernameError::LenCharMinViolated => "Username must be at least 4 characters long.",
            UsernameError::NotEmptyViolated => "Username cannot be empty.",
            UsernameError::LenCharMaxViolated => {
                "Username is too long. Please choose a shorter username."
            }
        },
        UserError::InvalidPassword(e) => match e {
            UnhashedPasswordError::LenCharMinViolated => {
                "Password must be at least 5 characters long."
            }
            UnhashedPasswordError::LenCharMaxViolated => "Password is too long.",
        },
    };

    (StatusCode::BAD_REQUEST, msg.to_string())
}

fn map_session_error(err: &SessionError) -> (StatusCode, String) {
    let (status_code, msg) = match err {
        SessionError::IncorrectPassword => {
            (StatusCode::FORBIDDEN, "username or password is incorrect")
        }
        SessionError::UserDoesNotExists => {
            (StatusCode::FORBIDDEN, "username or password is incorrect")
        }
        SessionError::InvalidTokenHash(_err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "an internal error ocurred",
        ),
        SessionError::InvalidSession => (
            StatusCode::UNAUTHORIZED,
            "there was a problem while authenticating the session",
        ),
    };

    (status_code, msg.to_string())
}
