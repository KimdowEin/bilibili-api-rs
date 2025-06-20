//! 视频简介

use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::traits::Data;

/// V1简介
 #[derive(Debug,Clone,PartialEq, Deserialize, Serialize,Data)]
pub struct  VideoDesc(String);
impl Deref for VideoDesc {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for VideoDesc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// V2简介
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoDesc2 {
    /// 简介内容
    pub raw_text: String,
    /// 类型
    #[serde(rename = "type")]
    pub desc_type: VideoDescType,
    /// 被@的用户mid
    pub biz_id: u64,
}

/// V2简介类型
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum VideoDescType {
    /// 普通
    Nomal = 1,
    /// 他人
    Human = 2,

    #[serde(other)]
    Unknown,
}
