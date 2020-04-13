use std::{str::FromStr, string::ToString};

use serde::{Deserialize, Serialize};

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

impl ToString for ListType {
    fn to_string(&self) -> String {
        match self {
            Self::Lunch => "lunch".to_string(),
            Self::Dinner => "dinner".to_string(),
        }
    }
}
