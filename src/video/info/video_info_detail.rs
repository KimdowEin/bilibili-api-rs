use serde::{Deserialize, Serialize};

use super::video_info::WebVideoInfoData;

/// 视频详细信息

pub const WEB_VIDEO_INFO_DETAIL_URL: &str =
    "https://api.bilibili.com/x/web-interface/wbi/view/detail";

#[derive(Debug, Serialize, Deserialize)]
pub struct WebVideoInfoDetailData {
    view: WebVideoInfoData,
    card: Card,
    tags: Tags,
    reply: Reply,
    related: Vec<Related>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    card: CardData,
    space: Sapce,
    following: bool,
    archive_count: u64,
    article_count: u64,
    follower: u64,
    like_num: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardData {
    mid: u64,
    name: String,
    sex: String,
    face: String,
    face_nft: u8,
    fans: u64,
    attention: u64,
    sign: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sapce {
    s_img: String,
    l_img: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reply {}
#[derive(Debug, Serialize, Deserialize)]
pub struct Related {}