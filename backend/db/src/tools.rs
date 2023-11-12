use diesel::prelude::*;

use anyhow::{bail, Result};
use thiserror::Error;

use super::models::{NewUser, User};
use super::schema::users;

#[derive(Debug, Error, PartialEq)]
pub enum UserError {
    #[error("incorrect password")]
    WrongPassword,
    #[error("user not found")]
    NotFoundUser,
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

pub fn get_password_salt_user(conn: &mut SqliteConnection, username: &str) -> Result<String> {
    Ok(search_user(conn, username)?.password_salt)
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
