use std::collections::HashMap;

use crate::model::user::{account::LiveAccountInfo, exp::MasterLevel};
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

/// https://gitee.com/KimdowEin/bilibili-API-collect/blob/master/docs/live/info.md#%E8%8E%B7%E5%8F%96%E6%88%BF%E9%97%B4%E9%A1%B5%E5%88%9D%E5%A7%8B%E5%8C%96%E4%BF%A1%E6%81%AF
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
}

/// 主播信息   
/// https://gitee.com/KimdowEin/bilibili-API-collect/blob/master/docs/live/info.md#%E8%8E%B7%E5%8F%96%E4%B8%BB%E6%92%AD%E4%BF%A1%E6%81%AF   
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
/// https://gitee.com/KimdowEin/bilibili-API-collect/blob/master/docs/live/info.md#%E6%89%B9%E9%87%8F%E6%9F%A5%E8%AF%A2%E7%9B%B4%E6%92%AD%E9%97%B4%E7%8A%B6%E6%80%81
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
}

/// 直播间信息
/// https://gitee.com/KimdowEin/bilibili-API-collect/blob/master/docs/live/info.md#%E8%8E%B7%E5%8F%96%E7%9B%B4%E6%92%AD%E9%97%B4%E4%BF%A1%E6%81%AF-1
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveRoomInfo {
    room_id: u64,
    short_id: u64,
    uid: u64,
    is_hidden: bool,
    is_locked: bool,
    is_portrait: bool,
    live_status: LiveStatus,
    hidden_till: u64,
    lock_till: u64,
    encrypted: bool,
    pwd_verified: bool,
    live_time: u64,
    room_shield: i64,
    playurl_info: Option<PlayUrlInfo>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayUrlInfo {
    conf_json: String,
    playurl:PlayUrl,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayUrl {
    cid:u64,
    g_qn_desc:Vec<QualityAndDesc>,
    stream:LiveStreamInfo,
    p2p_data:P2pData,
    dolby_qn:Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAndDesc{
    qn:LiveStreamQn,
    desc:String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStreamInfo {
    pub protocol_name:String,
    pub format:Vec<LiveStreamFormatDeltail>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStreamFormatDeltail {
    pub format_name:String,
    pub codec:Vec<LiveStreamCodecDetail>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStreamCodecDetail {
    pub codec_name:String,
    pub current_qn:LiveStreamQn,
    pub accept_qn:Vec<LiveStreamQn>,
    pub base_url:String,
    pub url_info:Vec<LiveStreamUrlInfo>,
    pub dolby_type:i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStreamUrlInfo {
    pub host:String,
    pub extra:String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct P2pData {
    pub p2p:bool,
    pub p2p_type:i64,
    pub m_p2p:bool,
    pub m_servers:Option<String>
}



#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LiveProtocol {
    Stream = 0,
    Hls = 1,
}
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LiveStreamFormat {
    Flv = 0,
    M3u8 = 1,
    Fmp4 = 2,
}
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LiveStreamCodec {
    AVC = 0,
    HEVC = 1,
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
}
