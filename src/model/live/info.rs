use std::collections::HashMap;

use crate::model::user::{
    account::{GenderType, LiveAccountInfo},
    exp::{LiveRoomLevel, MasterLevel},
    official::OfficialVerify,
    vip::VipType,
};
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_bool_from_anything;
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::stream::LiveStreamQn;

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveRoomView {
    pub uid: u64,
    pub room_id: u64,
    pub short_id: u64,
    pub attention: u64,
    pub online: u64,
    pub is_portrait: bool,
    pub description: String,
    pub live_status: u8,
    pub area_id: i32,
    pub parent_area_id: i32,
    pub background: String,
    pub title: String,
    pub user_cover: String,
    pub keyframe: String,
    pub is_strict_room: bool,
    pub live_time: String,
    pub tags: String,
    pub is_anchor: i64,
    pub room_silent_type: String,
    pub room_silent_level: i64,
    pub room_silent_second: u64,
    pub area_name: String,
    pub pardants: String,
    pub area_parents: String,
    pub hot_words: Vec<String>,
    pub hot_words_status: i64,
    pub pk_status: i64,
    pub pk_id: i64,
    pub battle_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveRoomInit {
    pub room_id: u64,
    pub short_id: u64,
    pub uid: u64,
    pub need_p2p: i64,
    pub is_hidden: bool,
    pub is_locked: bool,
    pub is_portrait: bool,
    pub live_status: LiveStatus,
    pub hidden_till: u64,
    pub lock_till: u64,
    pub encrypted: bool,
    pub pwd_verified: bool,
    pub live_time: i64,
    pub room_shield: i64,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_sp: bool,
    pub special_type: u8,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LiveRoomSpecialType {
    Normal = 0,
    Pay = 1,
    // 拜年祭特殊直播间
    NewYear = 2,

    #[serde(other)]
    Unknown,
}

/// 主播信息   
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveUserInfo {
    pub info: LiveAccountInfo,
    pub exp: MasterLevel,
    pub follower_num: u64,
    pub room_id: u64,
    pub medal_name: String,
    pub glory_count: u64,
    pub pendant: String,
    pub link_group_num: u64,
    pub room_news: LiveRoomNews,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveRoomNews {
    pub content: String,
    pub ctime: String,
    pub ctime_text: String,
}

/// 直播间基本信息   
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveRoomBaseInfoMap {
    by_room_ids: HashMap<String, LiveRoomBaseInfo>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveRoomBaseInfo {
    pub room_id: u64,
    pub uid: u64,
    pub area_id: u64,
    pub live_status: u8,
    pub live_url: String,
    pub parent_area_id: u64,
    pub title: String,
    pub parent_area_name: String,
    pub area_name: String,
    pub live_time: String,
    pub description: String,
    pub tags: String,
    pub attention: u64,
    pub online: u64,
    pub short_id: u64,
    pub uname: String,
    pub cover: String,
    pub background: String,
    pub join_slide: u8,
    pub live_id: u64,
    pub live_id_str: String,
}

/// 直播间状态   
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveRoomStatus {
    pub title: String,
    pub room_id: u64,
    pub uid: u64,
    pub online: u64,
    pub live_time: u64,
    pub live_status: LiveStatus,
    pub short_id: u64,
    pub area_v2_id: i64,
    pub area_v2_name: String,
    pub area_v2_parent_id: i64,
    pub area_v2_parent_name: String,
    pub uname: String,
    pub face: String,
    pub tag_name: String,
    pub tags: String,
    pub cover_from_user: String,
    pub keyframe: String,
    pub lock_till: String,
    pub hidden_till: String,
    pub broadcast_type: u8,
}
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LiveStatus {
    Close = 0,
    Live = 1,
    Round = 2,

    #[serde(other)]
    Unknown,
}

/// 直播间信息
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveRoomInfo {
    pub room_id: u64,
    pub short_id: u64,
    pub uid: u64,
    pub is_hidden: bool,
    pub is_locked: bool,
    pub is_portrait: bool,
    pub live_status: LiveStatus,
    pub hidden_till: u64,
    pub lock_till: u64,
    pub encrypted: bool,
    pub pwd_verified: bool,
    pub live_time: u64,
    pub room_shield: i64,
    pub playurl_info: Option<PlayUrlInfo>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayUrlInfo {
    pub conf_json: String,
    pub playurl: PlayUrl,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayUrl {
    pub cid: u64,
    pub g_qn_desc: Vec<QualityAndDesc>,
    pub stream: LiveStreamInfo,
    pub p2p_data: P2pData,
    pub dolby_qn: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAndDesc {
    pub qn: LiveStreamQn,
    pub desc: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStreamInfo {
    pub protocol_name: String,
    pub format: Vec<LiveStreamFormatDeltail>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStreamFormatDeltail {
    pub format_name: String,
    pub codec: Vec<LiveStreamCodecDetail>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStreamCodecDetail {
    pub codec_name: String,
    pub current_qn: LiveStreamQn,
    pub accept_qn: Vec<LiveStreamQn>,
    pub base_url: String,
    pub url_info: Vec<LiveStreamUrlInfo>,
    pub dolby_type: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStreamUrlInfo {
    pub host: String,
    pub extra: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct P2pData {
    pub p2p: bool,
    pub p2p_type: i64,
    pub m_p2p: bool,
    pub m_servers: Option<String>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LiveProtocol {
    Stream = 0,
    Hls = 1,

    #[serde(other)]
    Unknown,
}
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LiveStreamFormat {
    Flv = 0,
    M3u8 = 1,
    Fmp4 = 2,

    #[serde(other)]
    Unknown,
}
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LiveStreamCodec {
    AVC = 0,
    HEVC = 1,

    #[serde(other)]
    Unknown,
}

/// 获取直播间主播信息   
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveRoomOwner {
    pub info: LiveRoomOwnerInfo,
    pub level: LiveRoomLevel,
    pub san: u8,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveRoomOwnerInfo {
    pub uid: u64,
    pub uname: String,
    pub face: String,
    pub rank: String,
    pub platform_user_level: u64,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub mobile_verify: bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub identification: bool,
    pub official_verify: OfficialVerify,
    pub vip_type: VipType,
    pub gender: GenderType,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_deserialize_live_room_init() {
        let json = r#"{
            "room_id": 10209381,
            "short_id": 0,
            "uid": 296909317,
            "need_p2p": 0,
            "is_hidden": false,
            "is_locked": false,
            "is_portrait": false,
            "live_status": 2,
            "hidden_till": 0,
            "lock_till": 0,
            "encrypted": false,
            "pwd_verified": false,
            "live_time": -62170012800,
            "room_shield": 1,
            "is_sp": 0,
            "special_type": 0
        }"#;
        serde_json::from_str::<LiveRoomInit>(json).unwrap();
    }

    #[test]
    fn test_deserialize_live_room_info() {
        let json = r#"{
            "room_id": 10209381,
            "short_id": 0,
            "uid": 296909317,
            "is_hidden": false,
            "is_locked": false,
            "is_portrait": false,
            "live_status": 2,
            "hidden_till": 0,
            "lock_till": 0,
            "encrypted": false,
            "pwd_verified": true,
            "live_time": 0,
            "room_shield": 1,
            "all_special_types": [50],
            "playurl_info": null,
            "official_type": 0,
            "official_room_id": 0,
            "risk_with_delay": 0,
            "multi_screen_info": "",
            "pure_control_function": null,
            "degraded_playurl": null
        }"#;
        serde_json::from_str::<LiveRoomInfo>(json).unwrap();
    }

    #[test]
    fn test_deserialize_live_room_owner() {
        let json = r#"{
            "info": {
            "uid": 9617619,
            "uname": "哔哩哔哩直播",
            "face": "https://i0.hdslb.com/bfs/face/8f6a614a48a3813d90da7a11894ae56a59396fcd.jpg",
            "rank": "10000",
            "platform_user_level": 6,
            "mobile_verify": 1,
            "identification": 1,
            "official_verify": {
                "type": 1,
                "desc": "哔哩哔哩直播官方账号",
                "role": 3
            },
            "vip_type": 2,
            "gender": -1
            },
            "level": {
            "uid": 9617619,
            "cost": 7782673656,
            "rcost": 20199200291,
            "user_score": "0",
            "vip": 0,
            "vip_time": "0000-00-00 00:00:00",
            "svip": 0,
            "svip_time": "0000-00-00 00:00:00",
            "update_time": "2024-08-08 17:13:12",
            "master_level": {
                "level": 40,
                "color": 16746162,
                "current": [0, 147013810],
                "next": [0, 147013810],
                "anchor_score": 201992002,
                "upgrade_score": 0,
                "master_level_color": 16746162,
                "sort": "\u003E10000"
            },
            "user_level": 60,
            "color": 16752445,
            "anchor_score": 201992002
            },
            "san": 12
        }"#;
        serde_json::from_str::<LiveRoomOwner>(json).unwrap();
    }
}
