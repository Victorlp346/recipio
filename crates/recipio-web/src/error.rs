use axum::{
    Json,
    response::{IntoResponse, Response},
};
use recipio_core::{RecipioError, UserError};
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
            RecipioError::User(user_error) => match user_error {
                UserError::InvalidEmail(email_error) => match email_error {},
            },
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}
