use std::fmt;

const BASE_API_URL: &str = env!("BASE_API_URL");

pub enum AuthApi {
    Login,
    Refresh,
    Logout,
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
