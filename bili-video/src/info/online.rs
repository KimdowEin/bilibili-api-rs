//! 视频在线人数

use bili_core::{Data, ToQuery};
use serde::{Deserialize, Serialize};

use crate::VideoQuery;

/// 获取视频在线人数(web端)
pub const VIDEO_ONLINE_URL: &str = "https://api.bilibili.com/x/player/online/total";

/// 获取视频在线人数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct VideoOnlineQuery {
    #[serde(flatten)]
    pub vid: VideoQuery,
    pub cid: u64,
}

impl VideoOnlineQuery {
    pub fn new(vid: VideoQuery, cid: u64) -> Self {
        Self { vid, cid }
    }
}

/// 视频在线人数信息显示控制
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoOnlineShowSwitch {
    /// 是否展示所有终端总计人数
    pub total: bool,
    /// 是否展示web端实时在线人数
    pub count: bool,
}

/// 视频在线人数AB测试信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoOnlineAbtest {
    /// AB测试分组
    pub group: String,
}

/// 视频在线人数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct VideoOnline {
    /// 所有终端总计人数 例如`10万+`
    pub total: String,
    /// web端实时在线人数
    pub count: String,
    /// 数据显示控制
    pub show_switch: VideoOnlineShowSwitch,
    /// AB测试信息
    pub abtest: VideoOnlineAbtest,
}

#[cfg(test)]
mod tests {

    use bili_core::BiliResponse;

    use super::*;

    const BVID: &str = "BV1SWfwY3ENK";
    const CID: u64 = 29193274957;

    #[tokio::test]
    #[ignore]
    async fn test_query_video_online() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = VideoOnlineQuery {
            vid: BVID.into(),
            cid: CID,
        }
        .to_query()
        .unwrap()
        .to_url(VIDEO_ONLINE_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/video/video_online.json", &json)
            .await
            .unwrap();
    }

    #[test]
    fn test_deserialize_video_online() {
        let json = include_str!("../../../tests/datas/video/video_online.json");
        serde_json::from_str::<BiliResponse<VideoOnline>>(json).unwrap();
    }
}
