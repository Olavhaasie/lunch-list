use std::collections::HashMap;

use chrono::naive::NaiveDate;
use serde::{Deserialize, Serialize};

use super::list_type::ListType;

#[derive(Deserialize, Serialize)]
pub struct List {
    #[serde(rename = "type", flatten)]
    pub list_type: ListType,
    #[serde(rename = "date")]
    pub date: NaiveDate,
}

impl List {
    pub fn from_hash(hash: HashMap<String, String>) -> Option<Self> {
        if hash.is_empty() {
            return None;
        }

        let list_type = hash
            .get("type")
            .unwrap()
            .parse::<ListType>()
            .expect("list type must be either 'lunch' or 'dinner'");
        let date = hash
            .get("date")
            .unwrap()
            .parse::<NaiveDate>()
            .expect("date not formatted as ISO 8601");
        Some(Self { list_type, date })
    }
}
