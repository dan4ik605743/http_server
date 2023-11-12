use crate::api::crypto::secret_key;
use crate::api::network;
use crate::redis::RedisConnection;
use rand::Rng;
use redis::AsyncCommands;

pub async fn generate_unique_secret_key(username: &str) -> anyhow::Result<String> {
    let mut conn_redis = RedisConnection::get()
        .get_multiplexed_tokio_connection()
        .await?;

    let mut secret_key = generate_secret_key();
    let user_keys: Vec<String> = conn_redis.lrange(username, 0, -1).await?;

    if user_keys.is_empty() {
        return Ok(secret_key);
    }

    loop {
        if !user_keys.iter().any(|key| key == &secret_key) {
            break Ok(secret_key);
        } else {
            secret_key = secret_key::generate_secret_key();
        }
    }
}

//TODO
pub async fn verification_unique_secret_key(username: &str) -> anyhow::Result<()> {
    let mut conn_redis = RedisConnection::get()
        .get_multiplexed_tokio_connection()
        .await?;
    let user_keys: Vec<String> = conn_redis.lrange(username, 0, -1).await?;

    Ok(())
}

fn generate_secret_key() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const KEY_LENGTH: usize = 32;

    let mut rng = rand::thread_rng();
    (0..KEY_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
