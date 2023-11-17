use bb8_redis::{bb8::Pool, RedisConnectionManager};
use once_cell::sync::OnceCell;

type RedisPool = Pool<RedisConnectionManager>;
static REDIS_POOL: OnceCell<RedisPool> = OnceCell::new();

pub struct RedisConnection;
impl RedisConnection {
    pub fn get() -> &'static RedisPool {
        REDIS_POOL.get().expect("Redis pool is not initialized")
    }

    pub async fn set(redis_port: u32) {
        let manager = RedisConnectionManager::new(format!("redis://localhost:{}", redis_port))
            .expect("URL basic checks redis failed");
        let pool = Pool::builder()
            .build(manager)
            .await
            .expect("Could not build redis connection pool");

        // Checks redis online
        redis::Client::open(format!("redis://localhost:{}", redis_port))
            .unwrap()
            .get_connection()
            .expect("Failed connect to redis");

        REDIS_POOL.set(pool).unwrap();
    }
}
