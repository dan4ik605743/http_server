use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

type PasswordHashAndPasswordSalt = Result<(String, String)>;
type PasswordHash = Result<String>;

//TODO медленно? хуй знает
pub fn create_password_hash_and_password_salt(password: &str) -> PasswordHashAndPasswordSalt {
    let password_salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &password_salt)
        .map_err(|e| anyhow!(e))?
        .to_string();

    Ok((password_hash, password_salt.to_string()))
}

pub fn get_password_hash(password: &str, password_salt: &str) -> PasswordHash {
    let password_salt = SaltString::from_b64(password_salt).map_err(|e| anyhow!(e))?;

    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &password_salt)
        .map_err(|e| anyhow!(e))?
        .to_string())
}

//TODO
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn govno() -> Result<()> {
        let pass = "123";
        let (password_hash, password_salt) = create_password_hash_and_password_salt(pass)?;
        let resume_hash_password = get_password_hash(pass, &password_salt)?;
        assert_eq!(password_hash, resume_hash_password);
        Ok(())
    }
}
