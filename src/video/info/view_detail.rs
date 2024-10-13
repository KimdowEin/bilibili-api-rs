use serde::{Deserialize, Serialize};

use super::view::WebVideoInfoData;

/// 视频详细信息

pub const WEB_VIDEO_INFO_DETAIL_URL: &str =
    "https://api.bilibili.com/x/web-interface/wbi/view/detail";

#[derive(Debug, Serialize, Deserialize)]
pub struct WebVideoInfoDetailData {
    pub view: WebVideoInfoData,
    pub card: Card,
    pub tags: Tags,
    pub reply: Reply,
    pub related: Vec<Related>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub card: CardData,
    pub space: Sapce,
    pub following: bool,
    pub archive_count: u64,
    pub article_count: u64,
    pub follower: u64,
    pub like_num: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardData {
    pub mid: u64,
    pub name: String,
    pub sex: String,
    pub face: String,
    pub face_nft: u8,
    pub fans: u64,
    pub attention: u64,
    pub sign: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sapce {
    pub s_img: String,
    pub l_img: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reply {}
#[derive(Debug, Serialize, Deserialize)]
pub struct Related {}
