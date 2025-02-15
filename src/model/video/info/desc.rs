use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub type VideoDesc = String;


/// V2简介
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoDesc2 {
    /// 简介内容
    pub raw_text: String,
    /// 类型
    #[serde(rename = "type")]
    pub desc_type: u8,
    /// 被@的用户mid
    pub biz_id: u64,
}

/// V2简介类型
#[derive(Debug,Clone,Serialize_repr,Deserialize_repr,PartialEq)]
#[repr(u8)]
pub enum VideoDescType {
    /// 普通
    Nomal = 1,
    /// 他人
    Human = 2,
}
