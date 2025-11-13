use std::sync::LazyLock;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::{Algorithm, Argon2, Params, Version};

static ARGON2: LazyLock<Argon2> = LazyLock::new(|| {
    Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(
            Params::DEFAULT_M_COST,
            Params::DEFAULT_T_COST,
            Params::DEFAULT_P_COST,
            None,
        )
        .unwrap(),
    )
});

pub fn hash(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    ARGON2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!(e))
        .map(|v| v.to_string())
}

pub fn verify(password: &str, password_hash: &str) -> anyhow::Result<bool> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|e| anyhow::anyhow!(e))?;
    match ARGON2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(Error::Password) => Ok(false),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}
