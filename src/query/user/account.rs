use serde::{Deserialize, Serialize};

use crate::{Query, Sign};

/// 用户空间详细信息
pub const ACCOUNT_SPACE_INFO_URL: &str = "https://api.bilibili.com/x/space/wbi/acc/info";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Query, Sign)]
pub struct AccountSpaceInfoQuery {
    pub mid: u64,
}

/// 个人账号基本信息
pub const ACCOUNT_INFO_URL: &str = "https://api.bilibili.com/x/member/web/account";
/// 账号基本信息
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccountInfoQuery {
    #[serde(rename = "access_key")]
    pub app_access_key: Option<String>,
}
impl Query for AccountInfoQuery {}

impl AccountInfoQuery {
    pub fn new(app_access_key: Option<String>) -> Self {
        Self { app_access_key }
    }
}

/// 修改个性签名
pub const ACCOUNT_SIGN_UPDATE_URL: &str = "https://api.bilibili.com/x/member/web/sign/update";
/// 修改个性签名
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSignUpdateQuery {
    pub access_key: Option<String>,
    pub user_sign: String,
    pub csrf: Option<String>,
}
