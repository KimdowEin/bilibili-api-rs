//! 视频简介

use bili_core::Data;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::VideoQuery;

/// 获取视频简介
pub const VIDEO_DESC_URL: &str = "https://api.bilibili.com/x/web-interface/archive/desc";

/// 获取视频简介
pub type VideoDescQuery = VideoQuery;

/// V1简介
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Deref, DerefMut, Data)]
pub struct VideoDesc(String);

/// V2简介
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoDesc2 {
    /// 简介内容
    pub raw_text: String,
    /// 类型
    #[serde(rename = "type")]
    pub desc_type: VideoDescType,
    /// 被@的用户mid
    pub biz_id: u64,
}

/// V2简介类型
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum VideoDescType {
    /// 普通
    Nomal = 1,
    /// 他人
    Human = 2,

    #[serde(other)]
    Unknown,
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use bili_core::{BiliResponse, ToQuery};
    use bili_service::{Session, SessionState};
    use reqwest::ClientBuilder;
    use tokio::fs;

    use super::*;

    const BVID: &str = "BV1SWfwY3ENK";

    #[test]
    fn test_query_video_desc() {
        let url = VideoDescQuery::from(BVID)
            .to_query()
            .unwrap()
            .to_url(VIDEO_DESC_URL);

        assert_eq!(
            url,
            "https://api.bilibili.com/x/web-interface/archive/desc?bvid=BV1SWfwY3ENK"
        )
    }

    #[test]
    fn test_deserialize_video_desc() {
        let json = include_str!("../../../tests/datas/video_desc.json");
        serde_json::from_str::<BiliResponse<VideoDesc>>(json).unwrap();
    }

    #[tokio::test]
    async fn test_get_video_desc() {
        let state = SessionState::from_path("../cookies_v2.json")
            .map(Arc::new)
            .unwrap();
        let client = ClientBuilder::new()
            .cookie_provider(state.store.clone())
            .build()
            .unwrap();
        let session = Session::new(client, state);

        let url = VideoDescQuery::from(BVID)
            .to_query()
            .unwrap()
            .to_url(VIDEO_DESC_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        fs::write("../tests/datas/video_desc.json", &json)
            .await
            .unwrap();

        let desc = serde_json::from_str::<BiliResponse<VideoDesc>>(&json)
            .unwrap()
            .data()
            .unwrap();

        assert!(desc.starts_with("「あたしはまた弱虫モンブランだったみたいだ」"));
    }
}
