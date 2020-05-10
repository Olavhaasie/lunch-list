use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct ListsResponse {
    pub lists: Vec<List>,
}

#[derive(Debug, Deserialize)]
pub struct List {
    pub id: usize,
    pub date: String,
    #[serde(rename = "type")]
    pub list_type: String,
}
