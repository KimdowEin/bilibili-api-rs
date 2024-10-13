#![allow(dead_code)]

pub mod view;
pub mod view_detail;
pub mod archive_desc;
pub mod player_pagelist;


use std::fmt::Display;

use player_pagelist::PlayerPageListData;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use view::WebVideoInfoData;
use view_detail::WebVideoInfoDetailData;

#[derive(Debug,Serialize,Deserialize)]
pub struct InfoResponse {
    pub code:InfoResponseCode,
    pub message:String,
    ttl:u8,
    data:InfoData,
}
impl InfoResponse {
    pub fn take(self) -> InfoData {
        self.data
    }
}




#[derive(Debug,Serialize_repr,Deserialize_repr)]
#[repr(i32)]
pub enum InfoResponseCode {
    Success=0,
    RequestError=-400,
    PermissionDenied=-403,
    VideoNotFound=-404,
    VideoInvisible=62002,
    VideoAuditing=62004,
}
impl Display for InfoResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfoResponseCode::Success => write!(f,"成功"),
            InfoResponseCode::RequestError => write!(f,"请求错误"),
            InfoResponseCode::PermissionDenied => write!(f,"权限不足"),
            InfoResponseCode::VideoNotFound => write!(f,"视频不存在"),
            InfoResponseCode::VideoInvisible => write!(f,"视频不可见"),
            InfoResponseCode::VideoAuditing => write!(f,"视频审核中"),
        }
    }
    
}

#[derive(Debug,Serialize,Deserialize)]
pub enum InfoData {
    WebVideoInfoData(WebVideoInfoData),
    WebVideoInfoDetailData(WebVideoInfoDetailData),
    ArchiveDescData(String),
    PlayerPageListData(Vec<PlayerPageListData>),
}


// #[cfg(feature = "session")]

// mod session {
//     use super::*;
//     use crate::common::Session;
//     use crate::common::{Data, ResponseData};
//     use reqwest::Error;
//     use view::WEB_VIDEO_INFO_URL;

//     impl Session {
//         pub async fn get_web_video_info(&self, query: String) -> Result<WebVideoInfoData, Error> {
//             let url = format!("{}?{}", WEB_VIDEO_INFO_URL, query);
//             let response = self
//                 .get(url)
//                 .send()
//                 .await?
//                 .json::<ResponseData>()
//                 .await?
//                 .take();
//             if let Some(Data::WebVideoInfoData(data)) = response {
//                 Ok(data)
//             } else {
//                 panic!("Unexpected response type")
//             }
//         }
//     }
// }
// use serde::{Deserialize, Serialize};
// use serde_repr::{Deserialize_repr, Serialize_repr};
// #[cfg(feature = "session")]
// pub use session::*;

