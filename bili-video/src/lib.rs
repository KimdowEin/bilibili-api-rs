//! 视频查询

pub mod action;
pub mod archives;
pub mod format;
pub mod info;
pub mod newlist;
pub mod pbp;
pub mod popular;
pub mod ranking;
pub mod recommend;
pub mod report;
pub mod snapshot;
pub mod stream;
pub mod summary;

use bili_core::ToQuery;
use serde::{Deserialize, Serialize};

/// 通用视频查询
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, ToQuery)]
pub struct VideoQuery {
    pub aid: Option<u64>,
    pub bvid: Option<String>,
}
impl VideoQuery {
    pub fn new(aid: Option<u64>, bvid: Option<String>) -> Self {
        VideoQuery { aid, bvid }
    }
}
impl From<u64> for VideoQuery {
    fn from(value: u64) -> Self {
        VideoQuery::new(Some(value), None)
    }
}
impl From<&str> for VideoQuery {
    fn from(value: &str) -> Self {
        VideoQuery::new(None, Some(value.to_string()))
    }
}
impl From<String> for VideoQuery {
    fn from(value: String) -> Self {
        VideoQuery::new(None, Some(value))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_video_query() {
        let query = VideoQuery::new(Some(1), Some("BV1K54y1e7YP".to_string()));
        let query = query.to_query().unwrap();
        assert_eq!(query.inner(), "aid=1&bvid=BV1K54y1e7YP")
    }
}
