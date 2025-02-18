//! 投稿

use serde::{Deserialize, Serialize};

use crate::traits::Query;

pub const CONTRIBUTE_VIEW: &str = "https://api.bilibili.com/x/space/upstat";

// 查询用户播放量，阅读量，获赞数
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ContributeViewQuery {
    pub mid: u32,
}
impl ContributeViewQuery {
    pub fn new(mid: u32) -> Self {
        Self { mid }
    }
}
impl Query for ContributeViewQuery {}

pub const ALBUM_CONTRIBUTE_VIEW: &str = "https://api.vc.bilibili.com/link_draw/v1/doc/upload_count";

// 查询用户投稿数
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AlbumContributeViewQuery {
    pub uid: u32,
}
impl Query for AlbumContributeViewQuery {}

impl AlbumContributeViewQuery {
    pub fn new(uid: u32) -> Self {
        Self { uid }
    }
}
