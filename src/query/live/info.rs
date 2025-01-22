use serde::{Deserialize, Serialize};

use crate::{
    model::live::{info::{LiveProtocol, LiveStreamCodec, LiveStreamFormat}, stream::LiveStreamQn},
    traits::Query,
};

/// 获取直播间信息
pub const LIVE_ROOM_VIEW_URL: &str = "https://api.live.bilibili.com/room/v1/Room/get_info";

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveRoomViewQuery {
    room_id: u64,
}
impl Query for LiveRoomViewQuery {}
impl LiveRoomViewQuery {
    pub fn new(room_id: u64) -> Self {
        Self { room_id }
    }
}

/// 获取房间页初始化信息
pub const LIVE_ROOM_INIT_URL: &str = "https://api.live.bilibili.com/room/v1/Room/room_init";
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveRoomInitQuery {
    id: u64,
}
impl Query for LiveRoomInitQuery {}
impl LiveRoomInitQuery {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

/// 获取主播信息
pub const LIVE_USER_INFO_URL: &str = "https://api.live.bilibili.com/live_user/v1/Master/info";

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveUserInfoQuery {
    uid: u64,
}
impl Query for LiveUserInfoQuery {}
impl LiveUserInfoQuery {
    pub fn new(uid: u64) -> Self {
        Self { uid }
    }
}

/// 获取直播间基本信息
pub const LIVE_ROOM_BASE_INFO_URL: &str =
    "https://api.live.bilibili.com/xlive/web-room/v1/index/getRoomBaseInfo";

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveRoomBaseInfoQuery {
    req_biz: String,
    room_ids: Option<Vec<u64>>,
}
impl Query for LiveRoomBaseInfoQuery {}
impl LiveRoomBaseInfoQuery {
    pub fn new(req_biz: String, room_ids: Option<Vec<u64>>) -> Self {
        Self { req_biz, room_ids }
    }
}
/// 批量获取用户对应的直播间状态
pub const LIVE_ROOM_STATUS_URL: &str =
    "https://api.live.bilibili.com/room/v1/Room/get_status_info_by_uids";

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveRoomStatusQuery {
    uids: Vec<u64>,
}
impl Query for LiveRoomStatusQuery {}
impl LiveRoomStatusQuery {
    pub fn new(uids: Vec<u64>) -> Self {
        Self { uids }
    }
}

/// 获取直播间信息
pub const LIVE_ROOM_INFO_URL: &str =
    "https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo";

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveRoomInfoQuery {
    room_id: u64,
    protocol: LiveProtocol,
    format: LiveStreamFormat,
    codec: LiveStreamCodec,
    qn: LiveStreamQn,
    platform: String,
    ptype: u8,
    dolby: u8,
    panorama: u8,
}
impl Query for LiveRoomInfoQuery {}
impl LiveRoomInfoQuery {
    pub fn new(
        room_id: u64,
        protocol: LiveProtocol,
        format: LiveStreamFormat,
        codec: LiveStreamCodec,
        qn: LiveStreamQn,
    ) -> Self {
        let platform = "web".to_string();
        let ptype = 8;
        let dolby = 5;
        let panorama = 1;
        Self {
            room_id,
            protocol,
            format,
            codec,
            qn,
            platform,
            ptype,
            dolby,
            panorama,
        }
    }
}
