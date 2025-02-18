use serde::{Deserialize, Serialize};

use super::wbi::Wbi;

#[derive(Debug, Serialize, Deserialize)]
pub struct BiliTicket {
    pub ticket: String,
    pub created_at: u64,
    pub ttl: u64,
    #[serde(rename = "nav")]
    pub wbi: Wbi,
}
