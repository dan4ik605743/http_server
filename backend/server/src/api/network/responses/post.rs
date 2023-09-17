use axum::{response::Response, Json};
use axum_extra::extract::CookieJar;
use hyper::StatusCode;
use serde_json::Value;

pub type PostResponse<T> = Result<T, AppError>;
pub type JsonValue = Json<Value>;
pub type ResponseValue = Response<String>;

pub mod tools;

// AppErrorHandling
pub struct AppError(pub (anyhow::Error, Option<StatusCode>));

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            {
                tracing::error!("{}", self.0 .0);
                self.0 .1.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
                // StatusCode::INTERNAL_SERVER_ERROR
            },
            format!("Something went wrong: {}", self.0 .0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self((err.into(), Some(StatusCode::INTERNAL_SERVER_ERROR)))
    }
}
