use bcrypt::BcryptResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

impl Login {
    /// Returns true when the username is a valid username, false otherwise.
    pub fn validate(&self) -> bool {
        self.username
            .chars()
            .all(|c| c.is_alphanumeric() || c == ' ')
    }

    /// Returns true when the hash can be verified, false otherwise.
    pub fn verify_hash(&self, hash: &str) -> BcryptResult<bool> {
        bcrypt::verify(self.password.as_bytes(), hash)
    }

    pub fn hash(&self) -> BcryptResult<String> {
        bcrypt::hash(self.password.as_bytes(), bcrypt::DEFAULT_COST)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_username() {
        let login = Login {
            username: "Sir User".to_string(),
            password: "hunter2".to_string(),
        };
        assert!(login.validate());
    }

    #[test]
    fn test_username_with_special_characters() {
        let login = Login {
            username: "user#123?<>".to_string(),
            password: "hunter2".to_string(),
        };
        assert!(!login.validate());
    }
}
