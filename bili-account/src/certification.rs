//! 用户认证相关

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Official {
    /// todo
    pub role: u64,
    pub title: String,
    #[serde(flatten)]
    pub verify: OfficialVerify,
}

/// 认证信息
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OfficialVerify {
    #[serde(rename = "type")]
    pub is_verified: VerifiedType,
    pub desc: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize_repr, Serialize_repr)]
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
