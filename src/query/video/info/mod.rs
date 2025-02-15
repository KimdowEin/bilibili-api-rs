pub mod cids;
pub mod desc;
pub mod view;

use macros::{Query, Sign};
use serde::{Deserialize, Serialize};

use crate::traits::{Query, Sign};

#[derive(Debug, Clone, Deserialize, Serialize, Query, Sign)]
pub struct VideoQuery {
    pub aid: Option<u64>,
    pub bvid: Option<String>,
}
impl VideoQuery {
    pub fn new<I, S>(aid: I, bvid: S) -> Self
    where
        I: Into<Option<u64>>,
        S: Into<Option<String>>,
    {
        VideoQuery {
            aid: aid.into(),
            bvid: bvid.into(),
        }
    }
}
impl From<u64> for VideoQuery {
    fn from(value: u64) -> Self {
        VideoQuery::new(value, None)
    }
}
impl From<&str> for VideoQuery {
    fn from(value: &str) -> Self {
        VideoQuery::new(None, value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_query() {
        let query = VideoQuery::new(1, "BV1K54y1e7YP".to_string());
        let query = query.to_query().unwrap();
        assert_eq!(query, "aid=1&bvid=BV1K54y1e7YP")
    }
}
