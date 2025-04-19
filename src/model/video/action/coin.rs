//! 投币

use crate::Data;
use serde::{Deserialize, Serialize};

/// 投币返回
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct CoinVideo {
    pub like: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct IsCoin {
    pub multiply: u8,
}
