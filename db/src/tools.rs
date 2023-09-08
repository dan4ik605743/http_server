use std::cell::OnceCell;

use anyhow::Result;

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

use super::models::{NewUser, User};
use super::schema::users;

type Users = Vec<User>;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn get_connection_pool(db_path: &str) -> Result<OnceCell<Pool>> {
    let pool = Pool::builder().build(ConnectionManager::new(db_path))?;
    let cell = OnceCell::<Pool>::new();

    cell.set(pool).unwrap();
    Ok(cell)
}

pub fn create_user(conn: &mut SqliteConnection, username: &str, password: &str) -> Result<()> {
    let new_user = NewUser { username, password };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)?;

    Ok(())
}

pub fn get_users(conn: &mut SqliteConnection) -> Result<Users> {
    Ok(users::table.load::<User>(conn)?)
}
