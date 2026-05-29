use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub struct Created<T>(pub T);

impl<T> IntoResponse for Created<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self.0)).into_response()
    }
}
