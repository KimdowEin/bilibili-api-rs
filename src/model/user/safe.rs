use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSafe {
    pub account_info: AccountBindingInfo,
    pub account_safe: AccountSafeInfo,
    pub account_sns: AccountSnsInfo,
    pub account_other: AccountOtherInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBindingInfo {
    pub hide_tel: String,
    pub hide_mail: String,
    pub bind_tel: bool,
    pub bind_mail: bool,
    pub tel_verify: bool,
    pub mail_verify: bool,
    pub unneeded_check: bool,
    pub realname_certified: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSafeInfo {
    pub score_new: u32,
    pub pwd_level: u8,
    pub security: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSnsInfo {
    pub weibo_bind: bool,
    pub qq_bind: bool,
    pub wechat_bind: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountOtherInfo {
    #[serde(rename = "skipVerify")]
    pub skip_verify: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealNameCertified {
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealNameCertifiedDetail {
    pub status: u8,
    pub remake: String,
    pub realname: String,
    pub card: String,
    pub card_type: CardType,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CardType {
    IdCard = 0,
    HonKong = 2,
    TaiWan = 3,
    Passport = 4,
    PermanentResidence = 5,
    Other = 6,
}
