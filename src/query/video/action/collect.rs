//! 收藏

use serde::{Deserialize, Serialize};

use crate::{Csrf, Query};

/// 收藏视频（Web端）
pub const COLLECT_VIDEO_URL: &str = "https://api.bilibili.com/x/v3/fav/resource/deal";

/// 收藏视频（Web端）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Query, Csrf)]
pub struct CollectVideoQuery {
    pub rid: u64,
    #[serde(rename = "type")]
    pub type_: u8,
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
            type_: 2,
        }
    }
}

/// 判断视频是否被收藏（双端）
pub const IS_COLLECT_URL: &str = "https://api.bilibili.com/x/v2/fav/video/favoured";

/// 判断视频是否被收藏（双端）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Query)]
pub struct IsCollectQuery {
    pub aid: u64,
}
impl IsCollectQuery {
    pub fn new(aid: u64) -> Self {
        Self { aid }
    }
}
