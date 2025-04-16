//! 收藏

use serde::{Deserialize, Serialize};
use crate::Data;

// 收藏视频返回
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize,Data)]
pub struct CollectVideo {
    pub prompt: bool,
}

// 是否收藏视频
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize,Data)]
pub struct IsCollect {
    pub favoured: bool,
}
