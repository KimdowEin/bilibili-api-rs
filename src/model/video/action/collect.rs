//! 收藏

use crate::Data;
use serde::{Deserialize, Serialize};

// 收藏视频返回
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct CollectVideo {
    pub prompt: bool,
}

// 是否收藏视频
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct IsCollect {
    pub favoured: bool,
}
