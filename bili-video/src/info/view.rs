//! 视频信息
//!
//! 不要用VideoInfoQuery,容易被风控
//!
//! 建议将其内容分开请求

use bili_core::Data;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::{
    VideoQuery,
    info::{
        cids::CidItem,
        desc::{VideoDesc, VideoDesc2},
        state::{Dimension, Rights, UpowerState, VideoCopyRight, VideoStat, VideoState},
        subtitle::Subtitle,
    },
};

/// 获取视频概览
pub const VIDEO_VIEW_URL: &str = "https://api.bilibili.com/x/web-interface/wbi/view";
/// 获取视频概览
pub type VideoViewQuery = VideoQuery;

/// 视频信息概览
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct VideoView {
    /// 稿件avid
    pub aid: u64,
    /// 稿件bvid
    pub bvid: String,
    /// 视频1P的cid
    pub cid: u64,
    /// 稿件类型 1:原创 2:转载
    pub copyright: VideoCopyRight,
    /// 投稿时间
    #[serde(with = "chrono::serde::ts_seconds")]
    pub ctime: DateTime<Utc>,
    /// 视频简介
    pub desc: VideoDesc,
    /// 新版视频简介
    #[serde(deserialize_with = "deserialize_default_from_empty_object")]
    pub desc_v2: Vec<VideoDesc2>,
    /// 视频1P的分辨率
    pub dimension: Dimension,
    /// 未知
    pub disable_show_up_info: bool,
    /// 稿件总时长(所有分P)
    pub duration: u64,
    /// 视频同步发布的的动态的文字内容
    pub dynamic: String,
    /// 撞车视频跳转avid
    pub forward: Option<u64>,
    /// 未知
    pub is_chargeable_season: bool,
    /// 未知
    pub is_season_display: bool,
    /// 是否可以在 Story Mode 展示
    pub is_story: bool,
    /// 未知
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_story_play: bool,
    /// 充电信息
    #[serde(flatten)]
    pub upower: UpowerState,
    /// 私密
    pub is_view_self: bool,
    /// 稿件参加的活动id
    pub mission_id: Option<u64>,
    /// 未知
    pub need_jump_bv: bool,
    // /// UP主信息
    // pub owner: UserInfoBase,
    /// 视频分P列表
    pub pages: Vec<CidItem>,
    /// 封面图片url
    pub pic: String,
    /// 发布时间
    #[serde(with = "chrono::serde::ts_seconds")]
    pub pubdate: DateTime<Utc>,
    /// 视频属性标志
    pub rights: Rights,
    // 合作成员列表
    // todo
    // #[serde(default)]
    // pub staff: Vec<Staff>,
    /// 视频统计数据
    pub stat: VideoStat,
    /// 稿件状态
    pub state: VideoState,
    /// 视频CC字幕信息
    pub subtitle: Option<Subtitle>,
    /// 青少年模式(未知)
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub teenage_mode: bool,
    /// 分区tid
    pub tid: i64,
    /// 分区tid,文档还未解析
    pub tid_v2: i64,
    /// 稿件标题
    pub title: String,
    /// 子分区名称
    pub tname: String,
    /// 子分区名称
    pub tname_v2: String,
    // user_garb todo
    /// 稿件分P总数
    pub videos: u64,
}

// /// 视频超详细信息
// pub const VIDEO_INFO_URL: &str = "https://api.bilibili.com/x/web-interface/wbi/view/detail";
// /// 视频超详细信息
// pub type VideoInfoQuery = VideoQuery;

#[cfg(test)]
mod tests {

    use bili_core::{BiliResponse, ToQuery};

    use tokio::fs;

    use super::*;

    const BVID: &str = "BV1SWfwY3ENK";

    #[tokio::test]
    #[ignore]
    async fn test_query_video_view() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = VideoViewQuery::from(BVID)
            .to_query()
            .unwrap()
            .to_url(VIDEO_VIEW_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        fs::write("../tests/datas/video/video_view.json", &json)
            .await
            .unwrap();
    }

    #[test]
    fn test_deserialize_video_view() {
        let json = include_str!("../../../tests/datas/video/video_view.json");
        serde_json::from_str::<BiliResponse<VideoView>>(json).unwrap();
    }
}
