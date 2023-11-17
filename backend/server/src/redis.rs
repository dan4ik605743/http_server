use once_cell::sync::OnceCell;
use redis::Client;

type RedisClient = redis::Client;
static REDIS_CLIENT: OnceCell<Client> = OnceCell::new();

pub struct RedisConnection;
impl RedisConnection {
    pub fn get() -> &'static RedisClient {
        REDIS_CLIENT.get().expect("Redis pool is not initialized")
    }

    pub async fn set(redis_port: u32) -> anyhow::Result<()> {
        // Checks redis online
        redis::Client::open(format!("redis://localhost:{}", redis_port))
            .expect("URL basic checks redis failed")
            .get_connection()
            .expect("Failed connect to redis");

        let redis_client =
            redis::Client::open(format!("redis://localhost:{}", redis_port)).unwrap();
        REDIS_CLIENT
            .set(redis_client)
            .expect("Failed to set connection redis pool");
        Ok(())
    }
}
