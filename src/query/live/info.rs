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

pub const LIVE_ROOM_INIT_URL: &str = "https://api.live.bilibili.com/room/v1/Room/room_init";

pub struct LiveRoomInitQuery {
    id: u64,
}