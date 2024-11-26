//! 用户基本信息
//! 导航栏的用户信息都来自这里的请求

#![allow(dead_code)]
use std::fmt::Display;

use coin::CoinData;
use nav::NavData;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use stat::StatData;

use crate::common::Response;

pub mod coin;
pub mod nav;
pub mod stat;

/// 登陆基本信息响应结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct InfoResponse {
    code: InfoResponseCode,
    message: String,
    data: InfoData,
}

impl Response for InfoResponse {
    type Data = InfoData;
    fn is_success(&self) -> bool {
        self.code == InfoResponseCode::Success
    }
    fn message(self) -> String {
        self.message
    }
    fn data(self) -> Self::Data {
        self.data
    }
}

impl Display for InfoResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InfoData {
    NavData(NavData),
    StatData(StatData),
    CoinData(CoinData),
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum InfoResponseCode {
    Success = 0,
    APIKeyError = -3,
    NotLogin = -101,
    RequestError = -400,
}
impl Display for InfoResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfoResponseCode::Success => write!(f, "成功"),
            InfoResponseCode::APIKeyError => write!(f, "API校验密匙错误"),
            InfoResponseCode::NotLogin => write!(f, "账号未登录"),
            InfoResponseCode::RequestError => write!(f, "请求错误"),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::common::Session;

//     #[tokio::test]
//     async fn test_login_info_get_nav() {
//         Session::new().get_nav().await.unwrap();
//     }

//     #[tokio::test]
//     async fn test_login_info_stat() {}

//     #[tokio::test]
//     async fn test_login_info_get_coin() {}
// }
