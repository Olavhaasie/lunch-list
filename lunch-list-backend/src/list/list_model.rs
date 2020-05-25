use std::collections::{HashMap, HashSet};

use chrono::naive::NaiveDate;
use serde::{Deserialize, Serialize};

use super::list_type::ListType;

#[derive(Deserialize, Serialize)]
pub struct List {
    #[serde(skip_deserializing)]
    pub id: usize,
    #[serde(rename = "type", flatten)]
    pub list_type: ListType,
    #[serde(rename = "date")]
    pub date: NaiveDate,
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    size: Option<usize>,
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    users: Option<HashSet<String>>,
}

impl List {
    pub fn from_hash(id: usize, hash: HashMap<String, String>) -> Option<Self> {
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
        Some(Self {
            id,
            list_type,
            date,
            size: None,
            users: None,
        })
    }

    pub fn with_users(mut self, users: HashSet<String>) -> Self {
        self.users = Some(users);
        self
    }

    pub fn with_size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }
}
