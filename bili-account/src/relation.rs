//! 用户关系相关

use bili_core::ToQuery;
use serde::{Deserialize, Serialize};

/// 查询用户粉丝明细（新）
pub const USER_FOLLOWERS_URL: &str = "https://api.bilibili.com/x/relation/fans";
/// 查询用户关注明细（新）
pub const USER_FOLLOWINGS_URL: &str = "https://api.bilibili.com/x/relation/followings";

/// 查询用户粉丝明细
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct UserFollowersQuery {
    pub vmid: u64,
    pub pn: Option<u64>,
    pub ps: Option<u64>,
    pub offset: Option<String>,
    #[serde(rename = "from")]
    pub from_source: Option<String>,
}

/// 查询用户关注明细
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct UserFollowingsQuery {
    pub vmid: u64,
    pub pn: Option<u64>,
    pub ps: Option<u64>,
    pub order: Option<String>,
}

#[cfg(test)]
mod tests {

    use bili_core::ToQuery;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_query_user_followers() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = UserFollowersQuery {
            vmid: 296909317,
            pn: Some(1),
            ps: Some(50),
            offset: None,
            from_source: None,
        }
        .to_query()
        .unwrap()
        .to_url(USER_FOLLOWERS_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/user_followers.json", &json)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_query_user_followings() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = UserFollowingsQuery {
            vmid: 296909317,
            pn: Some(1),
            ps: Some(50),
            order: None,
        }
        .to_query()
        .unwrap()
        .to_url(USER_FOLLOWINGS_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/user_followings.json", &json)
            .await
            .unwrap();
    }
}
