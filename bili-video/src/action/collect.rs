//! 收藏

use bili_core::{Data, ToQuery};
use serde::{Deserialize, Serialize};

/// 收藏视频（Web端）
pub const COLLECT_VIDEO_URL: &str = "https://api.bilibili.com/x/v3/fav/resource/deal";

/// 收藏视频（Web端）
///
/// csrf
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct CollectVideoQuery {
    pub rid: u64,
    #[serde(rename = "type")]
    pub collect_type: u8,
    pub add_media_ids: Option<String>,
    pub del_media_ids: Option<String>,
}
impl CollectVideoQuery {
    pub fn new(rid: u64, add_media_ids: Option<Vec<u64>>, del_media_ids: Option<Vec<u64>>) -> Self {
        let add_media_ids = add_media_ids.map(|ids| {
            ids.into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",")
        });

        let del_media_ids = del_media_ids.map(|ids| {
            ids.into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",")
        });

        Self {
            rid,
            add_media_ids,
            del_media_ids,
            collect_type: 2,
        }
    }
}

// 收藏视频返回
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct CollectVideo {
    pub prompt: bool,
}

/// 判断视频是否被收藏（双端）
pub const IS_COLLECT_URL: &str = "https://api.bilibili.com/x/v2/fav/video/favoured";

/// 判断视频是否被收藏（双端）
///
/// query
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct IsCollectQuery {
    pub aid: u64,
}
impl IsCollectQuery {
    pub fn new(aid: u64) -> Self {
        Self { aid }
    }
}

// 是否收藏视频
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct IsCollect {
    pub favoured: bool,
}
