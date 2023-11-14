use crate::api::crypto::secret_key;

use super::prelude::*;

pub async fn register(Json(data): Json<JsonUser>) -> HandlerResponse<ResponseValue> {
    let conn_db = &mut SqliteConnection::get().get()?;

    if db::tools::search_user(conn_db, &data.username).is_ok() {
        return responses::send_err(
            format!(
                "A user with the same name: '{}' already exists",
                data.username
            ),
            StatusCode::CONFLICT,
        );
    }

    let password_struct = password::create_password_hash_and_password_salt(&data.password)?;

    match db::tools::create_user(
        conn_db,
        &data.username,
        &password_struct.password_hash,
        &password_struct.password_salt,
    ) {
        Ok(_) => {
            tracing::info!("Added user: '{}' to database", data.username);
            responses::send_ok()
        }
        Err(e) => responses::send_err(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn login(
    cookie: CookieValue,
    Json(data): Json<JsonUser>,
) -> HandlerResponse<CookieValue> {
    let conn_db = &mut SqliteConnection::get().get()?;

    let password_salt = match db::tools::get_password_salt_user(conn_db, &data.username) {
        Ok(val) => val,
        Err(e) if e.downcast_ref() == Some(&UserError::NotFoundUser) => {
            return responses::send_err(format!("'{}': {e}", data.username), StatusCode::NOT_FOUND);
        }
        Err(e) => {
            return responses::send_err(
                format!("'{}': {e}", data.username),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
        }
    };

    let password_hash = password::get_password_hash(&data.password, &password_salt)?;

    responses::send_err(secret_key::generate_secret_key(), StatusCode::ACCEPTED)
    // match db::tools::verification_user(conn_db, &data.username, &password_hash) {
    //     // Ok(_) => post::tools::send_json_response_ok(vec!["username"], vec![data.username]),
    //     // Ok(_) => post::tools::send_cookie_response_ok(cookie, "xxx", "field_data"),
    //     Ok(_) => session::create_session(cookie, &data.username).await,
    //     Err(e) => responses::send_err(
    //         format!("'{}': {e}", data.username),
    //         StatusCode::UNAUTHORIZED,
    //     ),
    // }
}
