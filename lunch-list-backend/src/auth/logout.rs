use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    #[serde(default)]
    pub all: bool,
}
