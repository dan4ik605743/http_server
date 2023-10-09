use crate::api::users::prelude::*;

pub async fn register(Json(data): Json<JsonUser>) -> PostResponse<ResponseValue> {
    let conn_db = &mut network::get_conn_db()?.get()?;

    if db::search_user(conn_db, &data.username).is_ok() {
        tracing::warn!(
            "A user with the same name: '{}' already exists",
            data.username,
        );
        return handlers_utils::send_err(StatusCode::CONFLICT);
    }

    let (password_hash, password_salt) =
        password::create_password_hash_and_password_salt(&data.password)?;

    match db::create_user(conn_db, &data.username, &password_hash, &password_salt) {
        Ok(_) => {
            tracing::info!("Added user: '{}' to database", data.username);
            handlers_utils::send_response_ok()
        }
        Err(e) => {
            tracing::error!("{e}");
            handlers_utils::send_err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn login(cookie: CookieValue, Json(data): Json<JsonUser>) -> PostResponse<CookieValue> {
    let conn_db = &mut network::get_conn_db()?.get()?;

    let password_salt = match db::get_salt(conn_db, &data.username) {
        Ok(val) => val,
        Err(e) if e.downcast_ref() == Some(&UserError::NotFoundUser) => {
            tracing::warn!("'{}': {e}", data.username);
            return handlers_utils::send_err(StatusCode::NOT_FOUND);
        }
        Err(e) => {
            tracing::warn!("'{}': {e}", data.username);
            return handlers_utils::send_err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let password_hash = password::get_password_hash(&data.password, &password_salt)?;

    match db::verification_user(conn_db, &data.username, &password_hash) {
        // Ok(_) => post::tools::send_json_response_ok(vec!["username"], vec![data.username]),
        Ok(_) => network::session::create_session(cookie, &data.username).await,
        // Ok(_) => post::tools::send_cookie_response_ok(cookie, "xxx", "field_data"),
        Err(e) => {
            tracing::warn!("'{}': {e}", data.username);
            handlers_utils::send_err(StatusCode::UNAUTHORIZED)
        }
    }
}
