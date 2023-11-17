use diesel::r2d2::{self, ConnectionManager};
use once_cell::sync::OnceCell;

type SqlitePool = r2d2::Pool<ConnectionManager<diesel::SqliteConnection>>;
static SQLITE_POOL: OnceCell<SqlitePool> = OnceCell::new();

pub struct SqliteConnection;
impl SqliteConnection {
    pub fn get() -> &'static SqlitePool {
        SQLITE_POOL.get().expect("Sqlite pool is not initialized")
    }

    pub fn set(db_path: &str) {
        let pool = SqlitePool::builder()
            .build(ConnectionManager::new(db_path))
            .expect("Could not build sqlite connection pool");

        SQLITE_POOL.set(pool).unwrap();
    }
}
