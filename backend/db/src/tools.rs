use std::cell::OnceCell;

use anyhow::{anyhow, bail, Result};
use thiserror::Error;

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

use super::models::{NewUser, User};
use super::schema::users;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug, Error, PartialEq)]
pub enum UserError {
    #[error("incorrect password")]
    WrongPassword,
    #[error("user not found")]
    NotFoundUser,
}

pub fn get_connection_pool(db_path: &str) -> Result<OnceCell<Pool>> {
    let pool = Pool::builder().build(ConnectionManager::new(db_path))?;
    let cell = OnceCell::<Pool>::new();

    cell.set(pool)
        .map_err(|_| anyhow!("Failed to set connection pool"))?;
    Ok(cell)
}

pub fn create_user(
    conn: &mut SqliteConnection,
    username: &str,
    password_hash: &str,
    password_salt: &str,
) -> Result<()> {
    let new_user = NewUser {
        username,
        password_hash,
        password_salt,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)?;

    Ok(())
}

pub fn verification_user(
    conn: &mut SqliteConnection,
    username: &str,
    password_hash: &str,
) -> Result<()> {
    if search_user(conn, username)?.password_hash == password_hash {
        Ok(())
    } else {
        bail!(UserError::WrongPassword);
    }
}

pub fn get_salt(conn: &mut SqliteConnection, username: &str) -> Result<String> {
    let found_user = search_user(conn, username)?;
    Ok(found_user.password_salt)
}

pub fn search_user(conn: &mut SqliteConnection, username: &str) -> Result<User> {
    let found_user = users::table
        .filter(users::username.eq(username))
        .first::<User>(conn)
        .optional()?;

    match found_user {
        Some(user) => Ok(user),
        None => bail!(UserError::NotFoundUser),
    }
}

// fn dbg_users(conn: &mut SqliteConnection) -> Result<()> {
//     let x = users::table.load::<User>(conn)?;
//     println!("{:#?}", x);
//     Ok(())
// }

// type Users = Vec<User>;
// pub fn get_users(conn: &mut SqliteConnection) -> Result<Users> {
//     Ok(users::table.load::<User>(conn)?)
// }
