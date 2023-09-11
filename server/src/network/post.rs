use axum::Json;

use db::{JsonUser, Pool, UserError};
use hyper::StatusCode;

mod hash;
mod result;
mod tools;

use result::{JsonStatusCode, PostJsonResult, PostResponseResult};

pub async fn register(Json(data): Json<JsonUser>, conn: Pool) -> PostResponseResult {
    let conn = &mut conn.get()?;

    if db::search_user(conn, &data.username).is_ok() {
        tracing::warn!(
            "A user with the same name: '{}' already exists",
            data.username,
        );
        return tools::return_response_error(StatusCode::CONFLICT);
    }

    let (password_hash, password_salt) =
        hash::create_password_hash_and_password_salt(&data.password)?;

    match db::create_user(conn, &data.username, &password_hash, &password_salt) {
        Ok(_) => {
            tracing::info!("Added user: '{}' to database", data.username);
            tools::return_response_ok()
        }
        Err(e) => {
            tracing::error!("{e}");
            tools::return_response_error(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn login(Json(data): Json<JsonUser>, conn: Pool) -> PostJsonResult {
    let conn = &mut conn.get()?;

    let password_salt = match db::get_salt(conn, &data.username) {
        Ok(val) => val,
        Err(e) if e.downcast_ref() == Some(&UserError::NotFoundUser) => {
            tracing::warn!("'{}': {e}", data.username);
            return tools::return_json_error(JsonStatusCode::NOT_FOUND);
        }
        Err(e) => {
            tracing::warn!("'{}': {e}", data.username);
            return tools::return_json_error(JsonStatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let password_hash = hash::get_password_hash(&data.password, &password_salt)?;

    match db::verification_user(conn, &data.username, &password_hash) {
        Ok(_) => tools::return_json_ok(vec!["username"], vec![data.username]),
        Err(e) => {
            tracing::warn!("'{}': {e}", data.username);
            tools::return_json_error(JsonStatusCode::UNAUTHORIZED)
        }
    }
}
