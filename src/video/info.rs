use serde::{Deserialize, Serialize};

use crate::session::Query;
#[cfg(feature = "session")]
mod session_use{
    pub use reqwest::Error;
    pub use crate::session::{Data,ResponseData};
    pub use crate::session::Session;
}
#[cfg(feature = "session")]
use session_use::*;

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
    // pub desc_v2: Desc,
    /// 稿件状态
    pub state: i64,
    /// 稿件总时长(所有分P)
    pub duration: u64,
    /// UP主信息
    pub owner: Owner,
    /// 视频同步发布的的动态的文字内容
    pub dynamic: String,
    /// 视频1P的cid
    pub cid: u64,
    /// 视频1P的分辨率
    #[serde(default)]
    pub dimension: Dimension,
    /// 视频分P列表
    pub pages: Vec<Page>,
    /// 合作成员列表
    #[serde(default)]
    pub staff: Vec<Staff>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Desc {
    pub raw_text: String,
    #[serde(rename = "type")]
    pub desc_type: u8,
    pub biz_id: u128,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub mid: u64,
    pub name: String,
    pub face: String,
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
    // pub vid: String,
    // /// 站外视频跳转链接
    // pub weblink: String,
    /// 分P分辨率
    #[serde(default)]
    pub dimension: Dimension,
}
#[derive(Debug, Serialize, Deserialize,Default)]
pub struct Dimension {
    pub width: u64,
    pub height: u64,
    pub rotate: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Staff {
    pub mid: u64,
    pub title: String, //名称
    pub name: String,  //昵称(暂时不知道有什么差别)
    pub face: String,
}

pub const WEB_VIDEO_INFO:&str = "https://api.bilibili.com/x/web-interface/wbi/view";

#[cfg(feature="session")]
impl Session {
    pub async fn get_web_video_info(&self, query: String) -> Result<WebVideoInfoData, Error> {
        let url = format!(
            "{}?{}",
            WEB_VIDEO_INFO, query
        );
        let response = self
            .get(url)
            .send()
            .await?
            .json::<ResponseData>()
            .await?
            .take();
        if let Some(Data::WebVideoInfoData(data)) = response {
            Ok(data)
        } else {
            panic!("Unexpected response type")
        }
    }
}

