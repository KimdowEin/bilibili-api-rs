//! 视频分P

use bili_core::Data;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

use crate::{VideoQuery, info::state::Dimension};

///查询视频分P列表 (avid/bvid转cid)
pub const VIDEO_CIDS_URL: &str = "https://api.bilibili.com/x/player/pagelist";

///查询视频分P列表 (avid/bvid转cid)
pub type VideoCidsQuery = VideoQuery;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, DerefMut, Data)]
pub struct VideoCids(Vec<CidItem>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CidItem {
    /// 视频分P的cid
    pub cid: u64,
    /// 分P序号
    pub page: u64,
    /// 视频来源 vupload：普通上传（B站）
    pub from: String,
    /// 分P标题
    pub part: String,
    /// 分P时长
    pub duration: u64,
    /// 站外视频vid
    pub vid: String,
    /// 站外视频跳转链接
    pub weblink: String,
    /// 分P分辨率
    pub dimension: Dimension,
    /// 封面,如果是VideoView则空
    pub first_frame: Option<String>,
    /// 上传时间
    pub ctime: u64,
}

#[cfg(test)]
mod tests {

    use bili_core::{BiliResponse, ToQuery};

    use tokio::fs;

    use super::*;

    const BVID: &str = "BV1SWfwY3ENK";

    #[tokio::test]
    #[ignore]
    async fn test_query_video_cids() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = VideoCidsQuery::from(BVID)
            .to_query()
            .unwrap()
            .to_url(VIDEO_CIDS_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        fs::write("../tests/datas/video/video_cids.json", &json)
            .await
            .unwrap();
    }

    #[test]
    fn test_deserialize_video_cids() {
        let json = include_str!("../../../tests/datas/video/video_cids.json");
        serde_json::from_str::<BiliResponse<VideoCids>>(json).unwrap();
    }
}
