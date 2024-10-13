#![allow(dead_code)]
///////////////////
/// 登录基本信息 ///
///////////////////
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

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoResponse {
    pub code: InfoResponseCode,
    pub message: String,
    pub data: Option<InfoData>,
}

impl Response for InfoResponse {
    type Data = InfoData; 
    fn is_success(&self) -> bool {
        self.code == InfoResponseCode::Success
    }
   fn message(&self) -> String {
       self.message.clone()
   } 
   fn data(self) -> Self::Data {
       self.data.unwrap()
   }
}

#[derive(Debug, Serialize, Deserialize)]
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

