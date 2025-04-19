use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Official {
    /// todo
    pub role: u64,
    pub title: String,
    #[serde(flatten)]
    pub verify: OfficialVerify,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct OfficialVerify {
    #[serde(rename = "type")]
    pub is_verified: VerifiedType,
    pub desc: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum VerifiedType {
    // 未认证
    NotVerified = -1,
    // 个人
    Personal = 0,
    // 企业
    Enterprise = 1,

    #[serde(other)]
    Unknown,
}
