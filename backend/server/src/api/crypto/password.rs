use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

type PasswordHash = Result<String>;
pub struct ArgonPassword {
    pub password_hash: String,
    pub password_salt: String,
}

pub fn create_password_hash_and_password_salt(password: &str) -> Result<ArgonPassword> {
    let password_salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &password_salt)
        .map_err(|e| anyhow!(e))?
        .to_string();

    Ok(ArgonPassword {
        password_hash,
        password_salt: password_salt.to_string(),
    })
}

pub fn get_password_hash(password: &str, password_salt: &str) -> PasswordHash {
    let password_salt = SaltString::from_b64(password_salt).map_err(|e| anyhow!(e))?;

    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &password_salt)
        .map_err(|e| anyhow!(e))?
        .to_string())
}
