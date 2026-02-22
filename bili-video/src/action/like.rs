//! 点赞

use bili_core::{Data, ToQuery};
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::VideoQuery;

// Web端点赞接口
pub const LIKE_VIDEO_URL: &str = "https://api.bilibili.com/x/web-interface/archive/like";

/// Web端点赞接口
///
/// csrf
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct LikeVideoQuery {
    #[serde(flatten)]
    pub vid: VideoQuery,
    pub like: u8,
}

impl LikeVideoQuery {
    pub fn new(vid: VideoQuery, like: bool) -> Self {
        let like = like.then_some(1).unwrap_or(2);
        Self { vid, like }
    }
}

/// 实际无返回，为了配合bilirequest trait而定义的LikeVideo类型
#[derive(Debug, Clone, Deref, DerefMut, PartialEq, Deserialize, Serialize, Data)]
pub struct LikeVideo(pub bool);

pub const IS_LIKE_VIDEO_URL: &str = "https://api.bilibili.com/x/web-interface/archive/has/like";

pub type IsLikeVideoQuery = VideoQuery;

#[derive(Debug, Clone, Deref, DerefMut, PartialEq, Deserialize, Serialize, Data)]
pub struct IsLikeVideo(#[serde(deserialize_with = "deserialize_bool_from_anything")] pub bool);
