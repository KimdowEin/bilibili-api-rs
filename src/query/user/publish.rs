// 发布相关，比如累计播放量

use crate::{traits::QueryTag,auth::AuthType};
use serde::{Deserialize, Serialize};

/// UP主状态数
pub const PUBLISH_UP_STAT_URL: &str = "https://api.bilibili.com/x/space/upstat";

/// UP主状态数
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, QueryTag)]
#[tag(Query)]
pub struct PublishUpStatQuery {
    pub mid: u64,
}
