use macros::Query;
use serde::{Deserialize, Serialize};

use crate::traits::Query;

pub const COLLECT_VIDEO_URL: &str = "https://api.bilibili.com/x/v3/fav/resource/deal";

#[derive(Debug,Clone, PartialEq, Serialize, Deserialize, Query)]
pub struct CollectVideoQuery {
    // pub access_key: Option<String>,
    pub rid: u64,
    #[serde(rename = "type")]
    pub type_: u8,
    pub add_media_ids: Option<String>,
    pub del_media_ids: Option<String>,
    pub csrf: String,
}
impl CollectVideoQuery {
    pub fn new(
        rid: u64,
        add_media_ids: Option<Vec<u64>>,
        del_media_ids: Option<Vec<u64>>,
        csrf: String,
    ) -> Self {
        let add_media_ids = add_media_ids.and_then(|ids| {
            let s = ids
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            Some(s)
        });

        let del_media_ids = del_media_ids.and_then(|ids| {
            let s = ids
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>()
                .join(",");
            Some(s)
        });

        Self {
            rid,
            add_media_ids,
            del_media_ids,
            csrf,
            type_: 2,
        }
    }

    pub fn add_media_ids(&mut self, add_media_ids: Vec<u64>) {
        self.add_media_ids = if let Some(media_ids) = &mut self.add_media_ids {
            media_ids.extend(add_media_ids.iter().map(|id| {
                let mut id = id.to_string();
                id.push(',');
                id
            }));
            Some(media_ids.to_owned())
        } else {
            let media_ids = add_media_ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            Some(media_ids)
        };
    }
    pub fn del_media_ids(&mut self, del_media_ids: Vec<u64>) {
        self.del_media_ids = if let Some(media_ids) = &mut self.del_media_ids {
            media_ids.extend(del_media_ids.iter().map(|id| {
                let mut id = id.to_string();
                id.push(',');
                id
            }));
            Some(media_ids.to_owned())
        } else {
            let media_ids = del_media_ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            Some(media_ids)
        };
    }
}

pub const IS_COLLECT_URL: &str = "https://api.bilibili.com/x/v2/fav/video/favoured";

#[derive(Debug,Clone, PartialEq, Serialize, Deserialize, Query)]
pub struct IsCollectQuery {
    pub aid: u64,
}
impl IsCollectQuery {
    pub fn new(aid: u64) -> Self {
        Self { aid }
    }
}
