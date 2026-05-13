//! 高能进度条

use bili_core::ToQuery;
use serde::{Deserialize, Serialize};

use crate::VideoQuery;

/// 获取弹幕趋势顶点列表
pub const PBP_DATA_URL: &str = "https://bvc.bilivideo.com/pbp/data";

/// 获取弹幕趋势顶点列表
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct PbpDataQuery {
    pub cid: u64,
    #[serde(flatten)]
    pub vid: Option<VideoQuery>,
}
impl PbpDataQuery {
    pub fn new(cid: u64, vid: Option<VideoQuery>) -> Self {
        Self { cid, vid }
    }
}

/// 弹幕趋势数据
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PbpData {
    /// 采样间隔时间(秒)
    pub step_sec: u64,
    pub tagstr: String,
    /// 数据本体
    pub events: PbpEvents,
    pub debug: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PbpEvents {
    /// 顶点值列表
    pub default: Vec<f64>,
}

#[cfg(test)]
mod tests {
    use bili_core::ToQuery;

    use tokio::fs;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_query_pbp_data() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = PbpDataQuery::new(3724723, None)
            .to_query()
            .unwrap()
            .to_url(PBP_DATA_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        fs::write("../tests/datas/video/pbp_data.json", &json)
            .await
            .unwrap();
    }

    #[test]
    fn test_deserialize_pbp_data() {
        let json = include_str!("../../tests/datas/video/pbp_data.json");
        serde_json::from_str::<PbpData>(json).unwrap();
    }
}
