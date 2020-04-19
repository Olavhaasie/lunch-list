use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    user_id: usize,
}

impl Claims {
    pub fn new(username: String, id: usize) -> Self {
        let date = Utc::now() + Duration::hours(1);
        Self {
            sub: username,
            exp: date.timestamp() as usize,
            user_id: id,
        }
    }
}
