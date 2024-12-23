use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveRoomInfo {
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
pub struct LiveRoomStatus{
    pub title: String,
    pub room_id: u64,
    pub uid: u64,
    pub online: u64,
    pub live_time:u64,
    pub live_status: u8,
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
    pub lock_till:String,
    pub hidden_till: String,
    pub broadcast_type: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveRoomNews{
    pub content: String,
    pub ctime: String,
    pub ctime_text: String,
}