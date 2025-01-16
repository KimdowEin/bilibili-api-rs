use serde::{Deserialize, Serialize};

pub type VideoDesc = String;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoDesc2 {
    /// 简介内容
    pub raw_text: String,
    /// 类型 1:普通 2:@他人
    #[serde(rename = "type")]
    pub desc_type: u8,
    /// 被@的用户mid
    pub biz_id: u64,
}

