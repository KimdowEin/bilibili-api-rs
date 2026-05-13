//! 视频快照

use bili_core::{Data, ToQuery};
use serde::{Deserialize, Serialize};

use crate::VideoQuery;

/// 获取视频快照（web端）
pub const VIDEO_SNAPSHOT_URL: &str = "https://api.bilibili.com/x/player/videoshot";

/// 获取视频快照（web端）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct VideoSnapshotQuery {
    #[serde(flatten)]
    pub vid: VideoQuery,
    pub cid: Option<u64>,
    pub index: Option<u8>,
}
impl VideoSnapshotQuery {
    pub fn new(vid: VideoQuery, cid: Option<u64>, index: Option<u8>) -> Self {
        Self { vid, cid, index }
    }
}

/// 视频快照信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct VideoSnapshot {
    /// bin格式截取时间表url
    pub pvdata: String,
    /// 每行图片数
    pub img_x_len: u64,
    /// 每列图片数
    pub img_y_len: u64,
    /// 每张图片长
    pub img_x_size: u64,
    /// 每张图片宽
    pub img_y_size: u64,
    /// 图片拼版url列表
    pub image: Vec<String>,
    /// json数组格式截取时间表
    pub index: Vec<u64>,
}

/// 获取视频快照（封面预览）
pub const VIDEO_SNAPSHOT_PREVIEW_URL: &str = "https://api.bilibili.com/pvideo";

/// 获取视频快照（封面预览）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct VideoSnapshotPreviewQuery {
    pub aid: u64,
}
impl VideoSnapshotPreviewQuery {
    pub fn new(aid: u64) -> Self {
        Self { aid }
    }
}

/// 视频快照预览信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct VideoSnapshotPreview {
    /// bin格式截取时间表url
    pub pvdata: String,
    /// 每行图片数
    pub img_x_len: u64,
    /// 每列图片数
    pub img_y_len: u64,
    /// 每张图片长
    pub img_x_size: u64,
    /// 每张图片宽
    pub img_y_size: u64,
    /// 图片拼版url列表
    pub image: Vec<String>,
    /// json数组格式截取时间表
    pub index: Vec<u64>,
}

#[cfg(test)]
mod tests {
    use bili_core::{BiliResponse, ToQuery};

    use tokio::fs;

    use super::*;

    const BVID: &str = "BV1SWfwY3ENK";

    #[tokio::test]
    #[ignore]
    async fn test_query_video_snapshot() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = VideoSnapshotQuery::new(BVID.into(), None, Some(1))
            .to_query()
            .unwrap()
            .to_url(VIDEO_SNAPSHOT_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        fs::write("../tests/datas/video/video_snapshot.json", &json)
            .await
            .unwrap();
    }

    #[test]
    fn test_deserialize_video_snapshot() {
        let json = include_str!("../../tests/datas/video/video_snapshot.json");
        serde_json::from_str::<BiliResponse<VideoSnapshot>>(json).unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_query_video_snapshot_preview() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = VideoSnapshotPreviewQuery::new(759949922)
            .to_query()
            .unwrap()
            .to_url(VIDEO_SNAPSHOT_PREVIEW_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        fs::write("../tests/datas/video/video_snapshot_preview.json", &json)
            .await
            .unwrap();
    }

    #[test]
    fn test_deserialize_video_snapshot_preview() {
        let json = include_str!("../../tests/datas/video/video_snapshot_preview.json");
        serde_json::from_str::<BiliResponse<VideoSnapshotPreview>>(json).unwrap();
    }
}
