#[cfg(test)]
mod tests {
    use super::super::password::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let pass = "123";
        let password_struct = create_password_hash_and_password_salt(pass)?;
        let resumed_hash_password = get_password_hash(pass, &password_struct.password_salt)?;

        assert_eq!(password_struct.password_hash, resumed_hash_password);
        Ok(())
    }
}
