use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountNotice {
    pub id: u64,
    pub content: String,
    pub url: String,
    pub notice_type: u8,
    pub icon: String,
    pub text_color: String,
    pub bg_color: String,
}
