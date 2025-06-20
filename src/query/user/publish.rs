// 发布相关，比如累计播放量

use crate::traits::Query;
use serde::{Deserialize, Serialize};

/// UP主状态数
pub const PUBLISH_UPSTAT_URL: &str = "https://api.bilibili.com/x/space/upstat";

/// UP主状态数
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Query)]
pub struct PublishUpStatQuery {
    pub mid: u64,
}
