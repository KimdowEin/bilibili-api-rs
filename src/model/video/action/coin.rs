use crate::Data;
use serde::{Deserialize, Serialize};

/// 登录响应数据
#[derive(Debug, Clone, PartialEq,Eq, Serialize, Deserialize,Data)]
pub struct CoinVideo {
    pub like: bool,
}

#[derive(Debug, Clone, PartialEq,Eq, Serialize, Deserialize,Data)]
pub struct IsCoin {
    pub multiply: u8,
}
