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

pub struct Success<T>(pub T);

impl<T> IntoResponse for Success<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self.0)).into_response()
    }
}
