use crate::{api::crypto::secret_key, redis::RedisConnection};
use anyhow::{bail, Result};
use axum_extra::extract::cookie::Cookie;
use redis::AsyncCommands;

use crate::api::handlers::responses::{CookieValue, HandlerResponse};

use time::Duration;

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

//TODO use Option<String>
pub async fn verification_session(cookie: CookieValue, username: &str) -> Result<()> {
    if cookie.get(username).is_none() {
        bail!("");
    }

    let mut conn_redis = RedisConnection::get()
        .get_multiplexed_tokio_connection()
        .await?;
    let secret_key: String = conn_redis.get(username).await?;
    if let Some(secret_key_cookie) = cookie.get(username) {
        if secret_key == secret_key_cookie.value() {
            Ok(())
        } else {
            bail!("");
        }
    } else {
        bail!("");
    }
}
