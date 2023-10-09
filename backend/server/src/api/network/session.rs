use crate::api::crypto::secret_key;
use crate::api::network;
use anyhow::{bail, Result};
use axum_extra::extract::cookie::Cookie;
use redis::AsyncCommands;

use super::handlers_utils::{CookieValue, PostResponse};

// use time::Duration;

pub async fn create_session(cookie: CookieValue, username: &str) -> PostResponse<CookieValue> {
    let mut conn_redis = network::get_conn_redis().await?;
    let secret_key = secret_key::generate_unique_secret_key(username).await?;

    conn_redis.rpush(username, &secret_key).await?;
    // conn_redis.expire(key, seconds)

    Ok(cookie.add(
        Cookie::build(username.to_string(), secret_key)
            // .max_age(Duration::hours(1))
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

    let mut conn_redis = network::get_conn_redis().await?;
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
