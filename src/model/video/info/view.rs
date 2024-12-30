use serde::{Deserialize, Serialize};

use crate::model::{user::account::{Owner, OwnerCard, Staff}, video::zone::Zone};

use super::{cids::Cids, desc::{VideoDesc, VideoDesc2}, state::{Dimension, Rights, VideoStat, VideoState}};


#[derive(Debug, Serialize, Deserialize)]
pub struct VideoView {
    /// 稿件bvid
    pub bvid: String,
    /// 稿件avid
    pub aid: u64,
    /// 稿件分P总数
    pub videos: u64,
    /// 分区tid
    pub tid: Zone,
    /// 子分区名称
    pub tname: String,
    /// 稿件类型 1:原创 2:转载
    pub copyright: u8,
    /// 封面图片url
    pub pic: String,
    /// 稿件标题
    pub title: String,
    /// 发布时间
    pub pubdate: u64,
    /// 投稿时间
    pub ctime: u64,
    /// 视频简介
    pub desc: VideoDesc,
    /// 新版视频简介
    pub desc_v2: Vec<VideoDesc2>,
    /// 稿件状态
    pub state: VideoState,
    /// 稿件总时长(所有分P)
    pub duration: u64,
    /// 撞车视频跳转avid
    pub forward: Option<u64>,
    /// 稿件参加的活动id
    pub mission_id: Option<u64>,
    /// 重定向url
    pub redirect_url: Option<String>,
    /// 视频属性标志
    pub rights: Rights,
    /// UP主信息
    pub owner: Owner,
    /// 视频状态数
    pub stat: VideoStat,
    /// 视频同步发布的的动态的文字内容
    pub dynamic: String,
    /// 视频1P的cid
    pub cid: u64,
    /// 视频1P的分辨率
    pub dimension: Dimension,
    /// 视频分P列表
    pub pages: Vec<Cids>,
    /// 视频CC字幕信息
    pub subtitle: Option<Subtitle>,
    /// 合作成员列表
    #[serde(default)]
    pub staff: Vec<Staff>,
}



#[derive(Debug, Default, Serialize, Deserialize, )]
pub struct Subtitle {
    ///是否允许提交字幕
    pub allow_submit: bool,
    /// 字幕列表
    pub list: Vec<SubtitleItem>,
}

#[derive(Debug, Default, Serialize, Deserialize, )]
pub struct SubtitleItem {
    /// 字幕id
    pub id: u64,
    /// 字幕语言
    pub lan: String,
    /// 字幕语言名称
    pub lan_doc: String,
    /// 是否锁定
    pub is_lock: bool,
    /// 作者mid
    pub author_mid: Option<u64>,
    /// json格式字幕文件url
    pub subtitle_url: String,

    // author
}



#[derive(Debug, Serialize, Deserialize)]
pub struct VideoInfo {
    /// 视频基本信息
    #[serde(rename = "View")]
    pub view: VideoView,
    #[serde(rename = "Card")]
    /// 视频up主信息
    pub card: OwnerCard,
    // /// 视频Tag信息
    // #[serde(rename = "Tags")]
    // pub tags: Tags,
    // /// 视频热评信息
    // #[serde(rename = "Reply")]
    // pub reply: Reply,
    // /// 视频相关推荐
    // #[serde(rename = "Related")]
    // pub related: Vec<Related>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reply {}
#[derive(Debug, Serialize, Deserialize)]
pub struct Related {}

