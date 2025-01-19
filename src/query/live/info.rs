use serde::{Deserialize, Serialize};

use crate::traits::Query;

/// 获取直播间信息
pub const LIVE_ROOM_INFO_URL: &str = "https://api.live.bilibili.com/room/v1/Room/get_info";

#[derive(Debug,Default, Serialize, Deserialize)]
pub struct LiveRoomInfoQuery {
    room_id: u64,
}
impl Query for LiveRoomInfoQuery {}
impl LiveRoomInfoQuery {
    pub fn new(room_id: u64) -> Self {
        Self {
            room_id,
        }
    }
}

/// 获取房间页初始化信息
pub const LIVE_ROOM_INIT_URL: &str = "https://api.live.bilibili.com/room/v1/Room/room_init";

pub struct LiveRoomInitQuery {
    id: u64,
}

/// 获取主播信息
pub const LIVE_USER_INFO_URL:&str = "https://api.live.bilibili.com/live_user/v1/Master/info";

#[derive(Debug,Default, Serialize, Deserialize)]
pub struct LiveUserInfoQuery {
    uid: u64,
}
impl Query for LiveUserInfoQuery {}


/// 获取直播间基本信息
pub const LIVE_ROOM_BASE_INFO_URL: &str = "https://api.live.bilibili.com/xlive/web-room/v1/index/getRoomBaseInfo";

#[derive(Debug,Default, Serialize, Deserialize)]
pub struct LiveRoomBaseInfoQuery {
    req_biz: String,
    room_ids: Option<Vec<u64>>,
}

/// 批量获取用户对应的直播间状态
pub const LIVE_ROOM_STATUS_URL: &str = "https://api.live.bilibili.com/room/v1/Room/get_status_info_by_uids";

#[derive(Debug,Default, Serialize, Deserialize)]
pub struct LiveRoomStatusQuery {
    uids: Vec<u64>,
}
impl Query for LiveRoomStatusQuery {}
impl LiveRoomStatusQuery {
    pub fn new(uids: Vec<u64>) -> Self {
        Self {
            uids,
        }
    }
}


