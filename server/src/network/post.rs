use axum::{response::Response, Json};
use serde_json::{json, Value};

use db::{JsonUser, Pool, UserError};
use hyper::StatusCode;

pub async fn register(
    Json(data): Json<JsonUser>,
    conn: Pool,
) -> Result<Response<String>, AppError> {
    let conn = &mut conn.get()?;

    match db::create_user(conn, &data.username, &data.password) {
        Ok(_) => {
            tracing::info!("Added user: '{}' to database", data.username);
            Ok(Response::new("OK".to_owned()))
        }
        Err(e) if e.to_string().contains("UNIQUE") => {
            tracing::warn!(
                "A user with the same name: '{}' already exists",
                data.username,
            );
            Ok(Response::builder()
                .status(StatusCode::CONFLICT)
                .body(String::new())?)
        }
        Err(e) => {
            tracing::error!("{e}");
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(String::new())?)
        }
    }
}

pub async fn login(Json(data): Json<JsonUser>, conn: Pool) -> Result<Json<Value>, AppError> {
    let conn = &mut conn.get()?;

    match db::verification_user(conn, &data.username, &data.password) {
        Ok(_) => Ok(Json(json!({ "message": "OK", "username": data.username }))),
        Err(e) if e.downcast_ref() == Some(&UserError::NotFoundUser) => {
            tracing::warn!("'{}': {e}", data.username);
            Ok(Json(json!({ "error": "Not Found" })))
        }
        Err(e) => {
            tracing::warn!("'{}': {e}", data.username);
            let x = &json!({"error": "Unauthorized"}).to_string();
            Ok(Json(json!({ "error": "Unauthorized" })))
        }
    }
}

// JsonErrorHandling

#[non_exhaustive]
struct PostJsonError;

impl PostJsonError {}

// AppErrorHandling
pub struct AppError(anyhow::Error);

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

// pub async fn login(Json(data): Json<JsonUser>, conn: Pool) -> Response<String> {
//     let conn = &mut conn.get().unwrap();

//     match db::verification_user(conn, &data.username, &data.password) {
//         Ok(_) => Response::new("OK".to_owned()),
//         Err(e) if e.downcast_ref() == Some(&UserError::NotFoundUser) => {
//             tracing::warn!("'{}': {e}", data.username);
//             Response::builder().status(404).body(String::new()).unwrap()
//         }
//         Err(e) => {
//             tracing::warn!("'{}': {e}", data.username);
//             Response::builder().status(401).body(String::new()).unwrap()
//         }
//     }
// }
