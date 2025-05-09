use serde::{Deserialize, Serialize};

use crate::{Query, Sign};

/// 获取视频合集信息   
pub const VIDEO_ARCHIVES_URL: &str =
    "https://api.bilibili.com/x/polymer/web-space/seasons_archives_list";

/// 获取视频合集信息   
#[derive(Debug, Serialize, Deserialize, Query, Sign)]
pub struct VideoArchiveQuery {
    /// 用户mid
    pub mid: u64,
    /// 视频合集ID
    pub season_id: u64,
    /// 页码索引
    pub page_num: u64,
    /// 每页视频数量
    pub page_size: u64,
    /// 排序方式，默认为false，true为倒序
    pub sort_reverse: Option<bool>,
    // pub gaia_vtoken: Option<String>,
    // pub web_location: Option<String>,
}
impl VideoArchiveQuery {
    pub fn new(
        mid: u64,
        season_id: u64,
        page_num: u64,
        page_size: u64,
        sort_reverse: Option<bool>,
        // gaia_vtoken: Option<String>,
    ) -> Self {
        // let web_location = Some("333.999".to_string());
        Self {
            mid,
            season_id,
            sort_reverse,
            page_num,
            page_size,
            // gaia_vtoken,
            // web_location,
        }
    }
}
