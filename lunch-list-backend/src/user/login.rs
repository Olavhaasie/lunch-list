use bcrypt::BcryptResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

impl Login {
    /// Returns true when the hash can be verified, false otherwise.
    pub fn verify_hash(&self, hash: &str) -> BcryptResult<bool> {
        bcrypt::verify(self.password.as_bytes(), hash)
    }

    pub fn hash(&self) -> BcryptResult<String> {
        bcrypt::hash(self.password.as_bytes(), bcrypt::DEFAULT_COST)
    }
}
