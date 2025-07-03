//! 视频查询

pub mod action;
// pub mod archives;
pub mod info;
pub mod stream;

use crate::{auth::AuthType, traits::QueryTag};
// use crate::traits::{Csrf, Query, Sign};
use serde::{Deserialize, Serialize};

/// 通用视频查询
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, QueryTag)]
#[tag(Query)]
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
        {}
        VideoQuery::new(None, value.to_string())
    }
}
impl From<String> for VideoQuery {
    fn from(value: String) -> Self {
        VideoQuery::new(None, value)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, QueryTag)]
#[tag(Sign)]
pub struct VideoSignQuery {
    pub aid: Option<u64>,
    pub bvid: Option<String>,
}
impl VideoSignQuery {
    pub fn new<I, S>(aid: I, bvid: S) -> Self
    where
        I: Into<Option<u64>>,
        S: Into<Option<String>>,
    {
        Self {
            aid: aid.into(),
            bvid: bvid.into(),
        }
    }
}
impl From<&str> for VideoSignQuery {
    fn from(value: &str) -> Self {
        VideoSignQuery::new(None, value.to_string())
    }
}
impl From<String> for VideoSignQuery {
    fn from(value: String) -> Self {
        VideoSignQuery::new(None, value)
    }
}
impl From<u64> for VideoSignQuery {
    fn from(value: u64) -> Self {
        VideoSignQuery::new(Some(value), None)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, QueryTag)]
#[tag(Csrf)]
pub struct VideoCsrfQuery {
    pub aid: Option<u64>,
    pub bvid: Option<String>,
}
impl VideoCsrfQuery {
    pub fn new<I, S>(aid: I, bvid: S) -> Self
    where
        I: Into<Option<u64>>,
        S: Into<Option<String>>,
    {
        Self {
            aid: aid.into(),
            bvid: bvid.into(),
        }
    }
}
impl From<u64> for VideoCsrfQuery {
    fn from(aid: u64) -> Self {
        Self::new(aid, None)
    }
}
impl From<String> for VideoCsrfQuery {
    fn from(value: String) -> Self {
        Self::new(None, Some(value))
    }
}
impl From<&str> for VideoCsrfQuery {
    fn from(value: &str) -> Self {
        Self::new(None, Some(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_video_query() {
    //     let query = VideoQuery::new(1, "BV1K54y1e7YP".to_string());
    //     let query = query.to_query().unwrap();
    //     assert_eq!(query, "aid=1&bvid=BV1K54y1e7YP")
    // }
}
