//! 视频流信息

use super::VideoQuery;
use crate::{
    auth::AuthType,
    model::video::stream::format::{Fnval, Qn},
    traits::QueryTag,
};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// 获取视频流地址
pub const VIDEO_STREAM_URL: &str = "https://api.bilibili.com/x/player/wbi/playurl";

/// 获取视频流地址
/// 
/// fnval为DASH时,qn无效
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, QueryTag,TypedBuilder)]
#[tag(Sign)]
pub struct VideoStreamQuery {
    #[serde(flatten)]
    pub vid: VideoQuery,
    pub cid: u64,
    #[builder(default,setter(into,strip_option))]
    pub qn: Option<Qn>,
    #[builder(default,setter(into,strip_option))]
    pub fnval: Option<Fnval>,

    #[builder(default,setter(transform = |fourk:bool| if fourk {Some(1)} else {Some(0)}))]
    pub fourk: Option<u8>,
    // session:String,
    // otype:String,
    // #[serde(rename="type")]
    // response_type:String,
    #[builder(default,setter(into,strip_option))]
    pub platform: Option<String>,
    // high_quality: Option<u8>,
}