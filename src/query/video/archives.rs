use serde::{Deserialize, Serialize};

use crate::traits::{Query, Sign};

/// 获取视频合集信息   
/// https://gitee.com/KimdowEin/bilibili-API-collect/blob/master/docs/video/collection.md#%E8%8E%B7%E5%8F%96%E8%A7%86%E9%A2%91%E5%90%88%E9%9B%86%E4%BF%A1%E6%81%AF
pub const VIDEO_ARCHIVES_URL: &str =
    "https://api.bilibili.com/x/polymer/web-space/seasons_archives_list";

/// 获取视频合集信息   
/// https://gitee.com/KimdowEin/bilibili-API-collect/blob/master/docs/video/collection.md#%E8%8E%B7%E5%8F%96%E8%A7%86%E9%A2%91%E5%90%88%E9%9B%86%E4%BF%A1%E6%81%AF
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoArchiveQuery {
    mid: u64,
    season_id: u64,
    page_num: u64,
    page_size: u64,
    sort_reverse: Option<bool>,
    gaia_vtoken: Option<String>,
    web_location: Option<String>,
}
impl Query for VideoArchiveQuery {}
impl Sign for VideoArchiveQuery {}
impl VideoArchiveQuery {
    pub fn new(
        mid: u64,
        season_id: u64,
        page_num: u64,
        page_size: u64,
        sort_reverse: Option<bool>,
        gaia_vtoken: Option<String>,
    ) -> Self {
        let web_location = Some("333.999".to_string());
        Self {
            mid,
            season_id,
            sort_reverse,
            page_num,
            page_size,
            gaia_vtoken,
            web_location,
        }
    }
}

