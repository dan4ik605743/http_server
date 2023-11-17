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
    let mut conn_redis = RedisConnection::get()
        .get_multiplexed_tokio_connection()
        .await?;

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
    let mut conn_redis = RedisConnection::get()
        .get_multiplexed_tokio_connection()
        .await?;

    let secret_key: String = conn_redis.get(username).await?;

    match cookie.get(username).unwrap().value() {
        secret_key_getted if secret_key == secret_key_getted => Ok(()),
        _ => bail!(SessionError::WrongSecretKey),
    }
}
