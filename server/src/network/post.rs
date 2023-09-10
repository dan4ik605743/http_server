use axum::Json;

use db::{JsonUser, Pool, UserError};
use hyper::StatusCode;

mod result;
mod tools;

use result::{JsonStatusCode, PostJsonResult, PostResponseResult};

pub async fn register(Json(data): Json<JsonUser>, conn: Pool) -> PostResponseResult {
    let conn = &mut conn.get()?;

    match db::create_user(conn, &data.username, &data.password) {
        Ok(_) => {
            tracing::info!("Added user: '{}' to database", data.username);
            tools::return_response_ok()
        }
        Err(e) if e.to_string().contains("UNIQUE") => {
            tracing::warn!(
                "A user with the same name: '{}' already exists",
                data.username,
            );
            tools::return_response_error(StatusCode::CONFLICT)
        }
        Err(e) => {
            tracing::error!("{e}");
            tools::return_response_error(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn login(Json(data): Json<JsonUser>, conn: Pool) -> PostJsonResult {
    let conn = &mut conn.get()?;

    match db::verification_user(conn, &data.username, &data.password) {
        Ok(_) => tools::return_json_ok(vec!["username"], vec![data.username]),
        Err(e) if e.downcast_ref() == Some(&UserError::NotFoundUser) => {
            tracing::warn!("'{}': {e}", data.username);
            tools::return_json_error(JsonStatusCode::NOT_FOUND)
        }
        Err(e) => {
            tracing::warn!("'{}': {e}", data.username);
            tools::return_json_error(JsonStatusCode::UNAUTHORIZED)
        }
    }
}
