#![allow(dead_code)]

pub mod archive_desc;
pub mod player_pagelist;
pub mod view;
pub mod view_detail;

use std::fmt::Display;

use player_pagelist::PlayerPageListData;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use view::WebVideoInfoData;
use view_detail::WebVideoInfoDetailData;

use crate::common::Response;

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoResponse {
    pub code: InfoResponseCode,
    pub message: String,
    ttl: u8,
    data: InfoData,
}
impl Response for InfoResponse {
    type Data = InfoData;
    fn is_success(&self) -> bool {
        self.code == InfoResponseCode::Success
    }
    fn message(self) -> String {
        self.message.clone()
    }
    fn data(self) -> Self::Data {
        self.data
    }
    
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum InfoResponseCode {
    Success = 0,
    RequestError = -400,
    PermissionDenied = -403,
    VideoNotFound = -404,
    VideoInvisible = 62002,
    VideoAuditing = 62004,
}
impl Display for InfoResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfoResponseCode::Success => write!(f, "成功"),
            InfoResponseCode::RequestError => write!(f, "请求错误"),
            InfoResponseCode::PermissionDenied => write!(f, "权限不足"),
            InfoResponseCode::VideoNotFound => write!(f, "视频不存在"),
            InfoResponseCode::VideoInvisible => write!(f, "视频不可见"),
            InfoResponseCode::VideoAuditing => write!(f, "视频审核中"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InfoData {
    WebVideoInfoData(WebVideoInfoData),
    WebVideoInfoDetailData(WebVideoInfoDetailData),
    ArchiveDescData(String),
    PlayerPageListData(Vec<PlayerPageListData>),
}

