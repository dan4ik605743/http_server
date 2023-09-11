use axum::{response::Response, Json};
use hyper::StatusCode;
use serde_json::Value;

pub type PostResponseResult = Result<Response<String>, AppError>;
pub type PostJsonResult = Result<Json<Value>, AppError>;

// JsonErrorHandling
#[non_exhaustive]
pub struct JsonStatusCode;

impl JsonStatusCode {
    pub const NOT_FOUND: &str = r#"{ "error": "Not Found" }"#;
    pub const UNAUTHORIZED: &str = r#"{ "error": "Unauthorized" }"#;
    pub const INTERNAL_SERVER_ERROR: &str = r#"{ "error": "Internal Server Error" }"#;
}

// AppErrorHandling
pub struct AppError(pub anyhow::Error);

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
