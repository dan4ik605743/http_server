#[cfg(test)]
mod tests {
    use super::super::passwords::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let pass = "123";
        let (password_hash, password_salt) = create_password_hash_and_password_salt(pass)?;
        let resumed_hash_password = get_password_hash(pass, &password_salt)?;

        assert_eq!(password_hash, resumed_hash_password);
        Ok(())
    }
}
