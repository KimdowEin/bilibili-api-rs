use serde::{Deserialize, Serialize};

/// CAPTCHA_URL响应Data
#[derive(Debug, Serialize, Deserialize)]
pub struct Captcha {
    pub token: String,
    pub geetest: Geetest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geetest {
    pub challenge: String,
    pub gt: String,
}

