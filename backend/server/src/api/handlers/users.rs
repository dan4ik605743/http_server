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

pub async fn login(auth_data: Auth<JsonUser>) -> HandlerResponse<CookieValue> {
    let conn_db = &mut SqliteConnection::get().get()?;
    let cookie = auth_data.extractors.cookie;
    let json = auth_data.extractors.json;

    let password_salt = match db::tools::get_password_salt_user(conn_db, &json.username) {
        Ok(val) => val,
        Err(e) if e.downcast_ref() == Some(&UserError::NotFoundUser) => {
            return responses::send_err(format!("'{}': {e}", json.username), StatusCode::NOT_FOUND)
        }
        Err(e) => {
            return responses::send_err(
                format!("'{}': {e}", json.username),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    };

    let password_hash = password::get_password_hash(&json.password, &password_salt)?;
    match db::tools::verification_user(conn_db, &json.username, &password_hash) {
        Ok(_) => {
            tracing::info!("'{}': Login successfuly", json.username);
            session::create_session(cookie, &json.username).await
        }
        Err(e) => responses::send_err(
            format!("'{}': {e}", json.username),
            StatusCode::UNAUTHORIZED,
        ),
    }
}

pub async fn user(auth_data: Auth<JsonUser>) -> HandlerResponse<HtmlPageResponse> {
    let cookie = auth_data.extractors.cookie;
    let json = auth_data.extractors.json;

    match session::verification_session(cookie, &json.username).await {
        Ok(_) => static_source::get_page(StaticSource::USER_PAGE).await,
        Err(e) if e.downcast_ref() == Some(&SessionError::NotFoundCookie) => {
            responses::send_err(format!("'{}': {e}", json.username), StatusCode::FORBIDDEN)
        }
        Err(e) if e.downcast_ref() == Some(&SessionError::WrongSecretKey) => responses::send_err(
            format!("'{}': {e}", json.username),
            StatusCode::UNAUTHORIZED,
        ),
        Err(e) => responses::send_err(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR),
    }
}
