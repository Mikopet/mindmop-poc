use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use jsend::JSendResponse;
use serde::Serialize;

/// Wraps the `JSend` and associates a valid HTTP status code to it
pub struct JsonResponse<T>
where
    T: Serialize,
{
    status_code: StatusCode,
    body: JSendResponse<T>,
}

impl<T: Serialize> JsonResponse<T> {
    /// Creates a `success` response with default HTTP status code 200
    fn success(data: T) -> Self {
        Self {
            status_code: StatusCode::OK,
            body: JSendResponse::success(Some(data)),
        }
    }

    /// Creates an `error` response with default HTTP status code 500
    fn error(message: String) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            body: JSendResponse::error(message, None, None),
        }
    }
}

impl<T, E> From<Result<T, E>> for JsonResponse<T>
where
    T: Serialize,
    E: std::error::Error,
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(v) => Self::success(v),
            Err(e) => Self::error(e.to_string()),
        }
    }
}

impl<T> IntoResponse for JsonResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (self.status_code, Json(self.body)).into_response()
    }
}
