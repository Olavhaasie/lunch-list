use std::cmp::min;

use serde::Deserialize;

const DEFAULT_FROM: usize = 0;
const MAX_QUERY_LEN: usize = 100;
const DEFAULT_QUERY_LEN: usize = 20;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    from: Option<usize>,
    len: Option<usize>,
    #[serde(default)]
    rev: bool,
}

impl ListQuery {
    pub fn to_range(&self) -> (usize, usize) {
        let start = self.from.unwrap_or(DEFAULT_FROM);
        let stop =
            (start + min(MAX_QUERY_LEN, self.len.unwrap_or(DEFAULT_QUERY_LEN))).saturating_sub(1);
        (start, stop)
    }

    pub fn rev(&self) -> bool {
        self.rev
    }
}
