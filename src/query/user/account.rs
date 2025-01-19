//! 账号信息

use serde::{Deserialize, Serialize};

use crate::traits::Query;

/// 个人账号基本信息
pub const ACCOUNT_MES_URL: &str = "https://api.bilibili.com/x/member/web/account";
/// 账号基本信息
#[derive(Debug,Default, Serialize, Deserialize)]
pub struct AccountInfoQuery {
    #[serde(rename = "access_key")]
    app_access_key: Option<String>,
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
    access_key: Option<String>,
    user_sign: String,
    csrf:Option<String>
}


