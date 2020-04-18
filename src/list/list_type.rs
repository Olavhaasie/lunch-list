use serde::{Deserialize, Serialize};

use std::{fmt, str::FromStr};

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ListType {
    #[serde(rename = "lunch")]
    Lunch,
    #[serde(rename = "dinner")]
    Dinner,
}

impl FromStr for ListType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lunch" => Ok(Self::Lunch),
            "dinner" => Ok(Self::Dinner),
            _ => Err(()),
        }
    }
}

impl fmt::Display for ListType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lunch => write!(f, "lunch"),
            Self::Dinner => write!(f, "dinner"),
        }
    }
}
