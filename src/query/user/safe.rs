use crate::traits::Query;
use serde::{Deserialize, Serialize};

pub const ACCOUNT_SAFE_URL: &str = "https://passport.bilibili.com/web/site/user/info";
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccountSafeQuery {
    pub access_key: Option<String>,
}
impl Query for AccountSafeQuery {}

pub const REALNAME_CERTIFIED: &str = "https://api.bilibili.com/x/member/realname/status";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RealNameCertifiedQuery {
    pub access_key: Option<String>,
}
impl Query for RealNameCertifiedQuery {}

pub const REALNAME_CERTIFIED_DETAIL: &str =
    "https://api.bilibili.com/x/member/realname/apply/status";
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RealNameCertifiedDetailQuery {
    pub access_key: Option<String>,
}
impl Query for RealNameCertifiedDetailQuery {}
