//! ticket 签名

use crate::traits::Data;
use serde::{Deserialize, Serialize};

use super::wbi::Wbi;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct BiliTicket {
    pub ticket: String,
    pub created_at: u64,
    pub ttl: u64,
    #[serde(rename = "nav")]
    pub wbi: Wbi,
}
