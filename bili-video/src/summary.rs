//! 视频AI总结

use bili_core::{Data, ToQuery};
use serde::{Deserialize, Serialize};

use crate::VideoQuery;

/// 获取AI总结内容
pub const VIDEO_SUMMARY_URL: &str = "https://api.bilibili.com/x/web-interface/view/conclusion/get";

/// 获取AI总结内容
///
/// sign
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct VideoSummaryQuery {
    #[serde(flatten)]
    pub vid: VideoQuery,
    pub cid: u64,
    pub up_mid: Option<u64>,
}
impl VideoSummaryQuery {
    pub fn new(vid: VideoQuery, cid: u64, up_mid: Option<u64>) -> Self {
        Self { vid, cid, up_mid }
    }
}

/// AI总结内容
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct VideoSummary {
    /// 返回值, -1: 不支持, 0: 有摘要, 1: 无摘要
    pub code: i32,
    /// 摘要内容
    pub model_result: Option<ModelResult>,
    /// 摘要id
    pub stid: String,
    pub status: i64,
    /// 点赞数
    pub like_num: u64,
    /// 点踩数
    pub dislike_num: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelResult {
    /// 数据类型, 0: 没有摘要, 1: 仅存摘要总结, 2: 摘要及提纲
    pub result_type: u8,
    /// 视频摘要
    pub summary: String,
    /// 分段提纲
    pub outline: Vec<OutlineItem>,
    /// AI字幕
    pub subtitle: Vec<SubtitlePart>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutlineItem {
    /// 分段标题
    pub title: String,
    /// 分段要点
    pub part_outline: Vec<PartOutline>,
    /// 分段起始时间(秒)
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartOutline {
    /// 要点起始时间(秒)
    pub timestamp: u64,
    /// 小结内容
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubtitlePart {
    /// 字幕分段
    pub part_subtitle: Vec<PartSubtitle>,
    /// 字幕识别起始时间(秒)
    pub timestamp: u64,
    /// 字幕标题
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartSubtitle {
    /// 字幕内容
    pub content: String,
    /// 分段开始时间(秒)
    pub start_timestamp: u64,
    /// 分段结束时间(秒)
    pub end_timestamp: u64,
}

#[cfg(test)]
mod tests {
    use bili_core::{BiliResponse, ToQuery};

    use tokio::fs;

    use super::*;

    const BVID: &str = "BV1L94y1H7CV";
    const CID: u64 = 1335073288;

    #[tokio::test]
    #[ignore]
    async fn test_query_video_summary() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = VideoSummaryQuery::new(BVID.into(), CID, Some(297242063))
            .to_query()
            .unwrap()
            .with_sign(&session.mixin_key())
            .unwrap()
            .to_url(VIDEO_SUMMARY_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        fs::write("../tests/datas/video/video_summary.json", &json)
            .await
            .unwrap();
    }

    #[test]
    fn test_deserialize_video_summary() {
        let json = include_str!("../../tests/datas/video/video_summary.json");
        serde_json::from_str::<BiliResponse<VideoSummary>>(json).unwrap();
    }
}
