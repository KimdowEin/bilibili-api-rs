use crate::traits::Data;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::{
    deserialize_bool_from_anything, deserialize_default_from_empty_object,
};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::official::OfficialVerify;

/// 关系状态数
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Data)]
pub struct RelationStat {
    /// 用户mid
    pub mid: u64,
    /// 关注数
    pub following: u64,
    /// 悄悄关注数
    pub whisper: u64,
    /// 黑名单
    pub black: u64,
    /// 粉丝数
    pub follower: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Data)]
pub struct RelationFollows {
    pub list: Vec<RelationItem>,
    #[serde(default)]
    pub total: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RelationItem {
    pub mid: u64,
    pub attribute: RelationType,
    pub mtime: u64,
    /// 分组id
    pub tag: Option<Vec<u64>>,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub special: bool,

    #[serde(deserialize_with = "deserialize_default_from_empty_object")]
    pub contract_info: ContractInfo,
    pub uname: String,
    pub face: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub face_nft: bool,
    pub sign: String,
    pub official_verify: OfficialVerify,
    // pub vip
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct ContractInfo {
    pub is_contract: bool,
    pub is_contractor: bool,
    pub ts: Option<u64>,
    pub user_attr: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum RelationType {
    UnFollow = 0,
    /// 悄悄关注（现已下线）
    Silent = 1,
    /// 已关注
    Following = 2,
    /// 已互粉
    Friend = 6,
    /// 已拉黑
    Black = 128,
}

/// 查询用户粉丝明细
pub type RelationFollowers = RelationFollows;

/// 查询用户粉丝明细
pub type RelationFollowings = RelationFollows;

#[derive(Debug, Clone, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum RelationModifyAction {
    Follow = 1,
    UnFollow = 2,
    #[deprecated(since = "1.0.0", note = "已下线")]
    Slient = 3,
    UnSlient = 4,
    Black = 5,
    UnBlack = 6,
    /// 踢出粉丝
    RidFollower,
}

#[derive(Debug, Clone, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum RelationModifyResource {
    UserSpace = 11,
    Video = 14,
    Article = 115,
    Event = 222,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserize_relation_stat() {
        let json = r#"{
        "mid":200435669,
        "following":990,
        "whisper":0,
        "black":14,
        "follower":1
        }"#;

        serde_json::from_str::<RelationStat>(json).unwrap();
    }

    #[test]
    fn test_deserize_relation_followers() {
        let json = r#"{
            "list": [{
                "mid": 6342150,
                "attribute": 0,
                "mtime": 1747585433,
                "tag": null,
                "special": 0,
                "contract_info": {},
                "uname": "逢魔之日",
                "face": "https://i0.hdslb.com/bfs/face/52aa6ae5dfe754c569d6567d7c494e4886ba80c9.jpg",
                "sign": "当DD不如单推api（",
                "face_nft": 0,
                "official_verify": {
                    "type": -1,
                    "desc": ""
                },
                "name_render": {},
                "nft_icon": "",
                "rec_reason": "",
                "track_id": "",
                "follow_time": ""
            }],
            "total": 274491
        }"#;

        serde_json::from_str::<RelationFollowers>(json).unwrap();
    }
}
