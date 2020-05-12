use std::fmt;

use yew::{format::Json, services::fetch};

const BASE_API_URL: &str = "/api";

pub type Response<T> = fetch::Response<Json<anyhow::Result<T>>>;

pub enum AuthApi {
    Login,
    Refresh,
    Logout,
}

pub enum ListApi {
    GetAll,
}

impl fmt::Display for AuthApi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/auth{}",
            BASE_API_URL,
            match self {
                Self::Login => "/login",
                Self::Refresh => "/refresh",
                Self::Logout => "/logout",
            }
        )
    }
}

impl fmt::Display for ListApi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/list{}",
            BASE_API_URL,
            match self {
                Self::GetAll => "",
            }
        )
    }
}
