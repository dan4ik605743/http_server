use axum_extra::extract::cookie::Cookie;

use anyhow::{bail, Result};
use redis::AsyncCommands;
use thiserror::Error;
use time::Duration;

use crate::api::handlers::responses::{CookieValue, HandlerResponse};
use crate::{api::crypto::secret_key, redis::RedisConnection};

#[derive(Debug, Error, PartialEq)]
pub enum SessionError {
    #[error("cookie not found")]
    NotFoundCookie,
    #[error("wrong secret key in cookie")]
    WrongSecretKey,
}

pub async fn create_session(cookie: CookieValue, username: &str) -> HandlerResponse<CookieValue> {
    let bindings = RedisConnection::get().clone();
    let mut conn_redis = bindings.get().await?;

    let secret_key = secret_key::generate_secret_key();

    conn_redis.set(username, &secret_key).await?;
    conn_redis.expire(username, 3600).await?;

    Ok(cookie.add(
        Cookie::build(username.to_string(), secret_key)
            .max_age(Duration::hours(1))
            .secure(true)
            .http_only(true)
            .finish(),
    ))
}

pub async fn verification_session(cookie: CookieValue, username: &str) -> Result<()> {
    if cookie.get(username).is_none() {
        bail!(SessionError::NotFoundCookie);
    }

    let bindings = RedisConnection::get().clone();
    let mut conn_redis = bindings.get().await?;

    let secret_key: String = conn_redis.get(username).await?;

    match cookie.get(username).unwrap().value() {
        secret_key_getted if secret_key == secret_key_getted => Ok(()),
        _ => bail!(SessionError::WrongSecretKey),
    }
}
