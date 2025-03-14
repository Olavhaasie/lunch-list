use std::{borrow::Cow, collections::HashMap};

use rand::Rng;
use serde::{Deserialize, Deserializer};
use validator::ValidationError;
use validator_derive::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct Login {
    #[serde(deserialize_with = "deserialize_username")]
    #[validate(
        length(min = 1, message = "Username cannot be empty"),
        custom = "validate_username"
    )]
    pub username: String,
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct Signup {
    #[serde(deserialize_with = "deserialize_username")]
    #[validate(
        length(min = 1, message = "Username cannot be empty"),
        custom = "validate_username"
    )]
    pub username: String,
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
    secret: String,
}

fn deserialize_username<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let username = String::deserialize(d)?;
    Ok(username.trim().to_string())
}

/// Returns true when the username is a valid username, false otherwise.
fn validate_username(username: &str) -> Result<(), ValidationError> {
    let valid = username.chars().all(|c| c.is_alphanumeric() || c == ' ');
    if valid {
        Ok(())
    } else {
        Err(ValidationError {
            code: Cow::from("username_validation"),
            message: Some(Cow::from(
                "Username can only contain alphanumeric characters and spaces",
            )),
            params: HashMap::new(),
        })
    }
}

impl Login {
    /// Returns true when the hash can be verified, false otherwise.
    pub fn verify_hash(&self, hash: &str) -> argon2::Result<bool> {
        argon2::verify_encoded(hash, self.password.as_bytes())
    }
}

impl Signup {
    pub fn verify_with_secret(&self, secret: &str) -> bool {
        self.secret == secret
    }

    pub fn hash(&self) -> argon2::Result<String> {
        let mut salt = [0u8; 32];
        rand::thread_rng().fill(&mut salt[..]);
        let config = argon2::Config::default();
        argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_valid_username() {
        let login = Login {
            username: "Sir User".to_string(),
            password: "hunter2".to_string(),
        };
        assert!(login.validate().is_ok());
    }

    #[test]
    fn test_empty_username() {
        let login = Login {
            username: "".to_string(),
            password: "hunter2".to_string(),
        };
        assert!(login.validate().is_err());
    }

    #[test]
    fn test_username_with_special_characters() {
        let login = Login {
            username: "user#123?<>".to_string(),
            password: "hunter2".to_string(),
        };
        assert!(login.validate().is_err());
    }
}
