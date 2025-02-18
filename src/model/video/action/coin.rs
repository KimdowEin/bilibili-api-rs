use serde::{Deserialize, Serialize};

/// 登录响应数据
#[derive(Debug, Serialize, Deserialize)]
pub struct CoinVideo {
    pub like: bool,
}
