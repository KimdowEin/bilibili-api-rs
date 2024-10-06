/// 视频基本信息

use serde::{Deserialize, Serialize};

use crate::common::Query;

pub const WEB_VIDEO_INFO_URL: &str = "https://api.bilibili.com/x/web-interface/wbi/view";

/// WEB_VIDEO_INFO_URL or WEB_VIDEO_INFO_DETAIL_URL
#[derive(Debug, Deserialize, Serialize)]
pub struct WebVideoInfoQuery {
    aid: Option<u64>,
    bvid: Option<String>,
}
impl WebVideoInfoQuery {
    pub fn new<N, S>(aid: N, bvid: S) -> Self
    where
        N: Into<Option<u64>>,
        S: Into<Option<String>>,
    {
        Self {
            aid: aid.into(),
            bvid: bvid.into(),
        }
    }
}
impl Query for WebVideoInfoQuery {}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebVideoInfoData {
    /// 稿件bvid
    pub bvid: String,
    /// 稿件avid
    pub aid: u64,
    /// 稿件分P总数
    pub videos: u64,
    /// 分区tid
    pub tid: u64,
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
    pub desc: String,
    /// 新版视频简介
    pub desc_v2: Vec<Desc>,
    /// 稿件状态
    pub state: i64,
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
    pub stat: Stat,
    /// 视频同步发布的的动态的文字内容
    pub dynamic: String,
    /// 视频1P的cid
    pub cid: u64,
    /// 视频1P的分辨率
    pub dimension: Dimension,
    /// 视频分P列表
    pub pages: Vec<Page>,
    /// 视频CC字幕信息
    pub subtitle: Subtitle,
    /// 合作成员列表
    #[serde(default)]
    pub staff: Vec<Staff>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Desc {
    /// 简介内容
    pub raw_text: String,
    /// 类型 1:普通 2:@他人
    #[serde(rename = "type")]
    pub desc_type: u8,
    /// 被@的用户mid
    pub biz_id: u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Rights {
    /// 是否允许承包
    pub bp: u8,
    /// 是否支持充电
    pub elec: u8,
    /// 是否支持下载
    pub download: u8,
    /// 是否电影
    pub movie: u8,
    /// 是否PGC付费
    pub pay: u8,
    /// 是否有高码率
    pub hd5: u8,
    /// 是否禁止转载
    pub no_reprint: u8,
    /// 是否自动播放
    pub autoplay: u8,
    /// 是否UGC付费
    pub ugc_pay: u8,
    /// 是否合作视频
    pub is_cooperation: u8,
    /// 是否互动视频
    pub is_stein_gate: u8,
    /// 是否全景视频
    pub is_360: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    /// UP mid
    pub mid: u64,
    /// UP昵称
    pub name: String,
    /// UP头像
    pub face: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    /// 稿件avid
    pub aid: u64,
    /// 播放数
    pub view: u64,
    /// 评论数
    pub reply: u64,
    /// 弹幕数
    pub danmaku: u64,
    /// 点赞数
    pub like: u64,
    /// 投币数
    pub coin: u64,
    /// 收藏数
    pub favorite: u64,
    /// 分享数
    pub share: u64,
    /// 当前排名
    pub now_rank: u64,
    /// 历史最高排名
    pub his_rank: u64,
    /// 点踩数
    pub dislike: u64,
    /// 视频评分
    pub evaluation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    /// 视频分P的cid
    pub cid: u64,
    /// 分P序号
    pub page: u64,
    /// 视频来源 vupload：普通上传（B站）
    pub from: String,
    /// 分P标题
    pub part: String,
    /// 分P时长
    pub duration: u64,
    // /// 站外视频vid
    pub vid: String,
    // /// 站外视频跳转链接
    pub weblink: String,
    /// 分P分辨率
    pub dimension: Dimension,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subtitle {
    ///是否允许提交字幕
    allow_submit: bool,
    /// 字幕列表
    list: Vec<SubtitleItem>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubtitleItem {
    /// 字幕id
    id: u64,
    /// 字幕语言
    lan: String,
    /// 字幕语言名称
    lan_doc: String,
    /// 是否锁定
    is_lock: bool,
    /// 作者mid
    author_mid: u64,
    /// json格式字幕文件url
    subtitle_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Dimension {
    pub width: u64,
    pub height: u64,
    /// 反转 0：正常 1：反转
    pub rotate: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Staff {
    pub mid: u64,
    pub title: String, //名称
    pub name: String,  //昵称(暂时不知道有什么差别)
    pub face: String,
}
