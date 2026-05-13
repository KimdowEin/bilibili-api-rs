//! 用户基本信息

use bili_core::ToQuery;
use serde::{Deserialize, Serialize};

/// 用户名片信息
pub const USER_CARD_URL: &str = "https://api.bilibili.com/x/web-interface/card";

/// 用户名片信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct UserCardQuery {
    pub mid: u64,
    pub photo: Option<bool>,
}

#[cfg(test)]
mod tests {

    use bili_core::ToQuery;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_query_user_card() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = UserCardQuery {
            mid: 296909317,
            photo: Some(false),
        }
        .to_query()
        .unwrap()
        .to_url(USER_CARD_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/user_card.json", &json)
            .await
            .unwrap();
    }
}
