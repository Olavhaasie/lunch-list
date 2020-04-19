use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}
