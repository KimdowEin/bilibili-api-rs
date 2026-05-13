//! 投币

use bili_core::{Data, ToQuery};
use serde::{Deserialize, Serialize};

use crate::VideoQuery;

/// 投币视频（web端）
pub const COIN_VIDEO_URL: &str = "https://api.bilibili.com/x/web-interface/coin/add";

/// 投币视频（web端）
///
/// csrf(post)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct CoinVideoQuery {
    #[serde(flatten)]
    pub vid: VideoQuery,
    /// 投币数量
    pub multiply: u8,
    /// 附带点赞
    pub select_like: u8,
}
impl CoinVideoQuery {
    pub fn new(vid: VideoQuery, coin_two: bool, then_like: bool) -> Self {
        let multiply = coin_two.then_some(2).unwrap_or(1);
        let select_like = then_like.then_some(1).unwrap_or(0);
        Self {
            vid,
            multiply,
            select_like,
        }
    }
}

/// 投币返回
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct CoinVideo {
    pub like: bool,
}

/// 查询是否投币
pub const IS_COIN_URL: &str = "https://api.bilibili.com/x/web-interface/archive/coins";

/// 查询是否投币
pub type IsCoinQuery = VideoQuery;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct IsCoin {
    pub multiply: u8,
}

#[cfg(test)]
mod tests {

    use bili_core::BiliResponse;

    use tokio::fs;

    use super::*;

    const BVID: &str = "BV1uSfLB3E21";

    #[tokio::test]
    #[ignore = "每次运行都要改变BVID，否则code不为0"]
    async fn test_query_coin_video() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");
        session.refresh_csrf().unwrap();

        let base_query = VideoQuery::from(BVID);
        let url = CoinVideoQuery::new(base_query, false, true)
            .to_query()
            .unwrap()
            .with_csrf(&session.bili_jct())
            .unwrap()
            .to_url(COIN_VIDEO_URL);

        let json = session
            .post(url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        fs::write("../tests/datas/video/coin_video.json", &json)
            .await
            .unwrap();
    }

    #[test]
    #[ignore = "应对query时没有改BVID"]
    fn test_deserialize_coin_video() {
        let json = include_str!("../../../tests/datas/video/coin_video.json");
        serde_json::from_str::<BiliResponse<CoinVideo>>(json).unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_query_is_coin() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");
        session.refresh_csrf().unwrap();

        let url = IsCoinQuery::from(BVID)
            .to_query()
            .unwrap()
            .to_url(IS_COIN_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        fs::write("../tests/datas/video/is_coin.json", &json)
            .await
            .unwrap();
    }

    #[test]
    fn test_deserialize_is_coin() {
        let json = include_str!("../../../tests/datas/video/is_coin.json");
        serde_json::from_str::<BiliResponse<IsCoin>>(json).unwrap();
    }
}
