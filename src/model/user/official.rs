use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Official {
    /// todo
    pub role: u64,
    pub title: String,
    pub desc: String,
    #[serde(rename = "type")]
    pub is_verified: u8,
}
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OfficialVerify {
    #[serde(rename = "type")]
    pub is_verified: i8,
    pub desc: String,
}


