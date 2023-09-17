use axum_extra::extract::CookieJar;

use crate::api::users::prelude::*;

pub async fn register(Json(data): Json<JsonUser>, conn: Pool) -> PostResponse<ResponseValue> {
    let conn = &mut conn.get()?;

    if db::search_user(conn, &data.username).is_ok() {
        tracing::warn!(
            "A user with the same name: '{}' already exists",
            data.username,
        );
        // return tools::send_response_error(StatusCode::CONFLICT);
        return tools::send_err(StatusCode::CONFLICT);
    }

    let (password_hash, password_salt) =
        passwords::create_password_hash_and_password_salt(&data.password)?;

    match db::create_user(conn, &data.username, &password_hash, &password_salt) {
        Ok(_) => {
            tracing::info!("Added user: '{}' to database", data.username);
            tools::send_response_ok()
        }
        Err(e) => {
            tracing::error!("{e}");
            tools::send_err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// pub async fn test(cookie: CookieJar, conn: Pool) -> PostCookieResponse {
//     let conn = &mut conn.get()?;
//     tools::send_cookie_response_err(StatusCode::INTERNAL_SERVER_ERROR)
// }

pub async fn login(Json(data): Json<JsonUser>, conn: Pool) -> PostResponse<JsonValue> {
    let conn = &mut conn.get()?;

    let password_salt = match db::get_salt(conn, &data.username) {
        Ok(val) => val,
        Err(e) if e.downcast_ref() == Some(&UserError::NotFoundUser) => {
            tracing::warn!("'{}': {e}", data.username);
            return tools::send_err(StatusCode::NOT_FOUND);
        }
        Err(e) => {
            tracing::warn!("'{}': {e}", data.username);
            return tools::send_err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let password_hash = passwords::get_password_hash(&data.password, &password_salt)?;

    match db::verification_user(conn, &data.username, &password_hash) {
        Ok(_) => tools::send_json_response_ok(vec!["username"], vec![data.username]),
        Err(e) => {
            tracing::warn!("'{}': {e}", data.username);
            tools::send_err(StatusCode::UNAUTHORIZED)
        }
    }
}
