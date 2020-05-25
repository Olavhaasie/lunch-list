use chrono::NaiveDate;
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
pub struct ListResponse {
    pub id: usize,
    pub date: NaiveDate,
    #[serde(rename = "type")]
    pub list_type: String,
    pub users: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct List {
    pub id: usize,
    pub date: NaiveDate,
    #[serde(rename = "type")]
    pub list_type: String,
}
