use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::users;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,

    pub password_hash: String,
    pub password_salt: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,

    pub password_hash: &'a str,
    pub password_salt: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonUser {
    pub username: String,
    pub password: String,
}
