use bili_core::{Data, ToQuery};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// 获取视频合集信息
pub const VIDEO_ARCHIVES_URL: &str =
    "https://api.bilibili.com/x/polymer/web-space/seasons_archives_list";

/// 获取视频合集信息
///
/// sign
#[derive(Debug, Serialize, Deserialize, TypedBuilder, ToQuery)]
pub struct VideoArchiveQuery {
    /// 用户mid
    pub mid: u64,
    /// 视频合集ID
    pub season_id: u64,
    /// 页码索引
    #[builder(default = 1)]
    pub page_num: u64,
    /// 每页视频数量
    #[builder(default = 30)]
    pub page_size: u64,
    /// 排序方式，默认为false，true为倒序
    #[builder(default)]
    pub sort_reverse: bool,
    // pub gaia_vtoken: Option<String>,
    // pub web_location: Option<String>,
}

/// 获取视频合集信息
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Data)]
pub struct VideoArchive {
    /// 稿件avid
    pub aids: Vec<u64>,
    /// 合集视频
    pub archives: Vec<VideoArchiveItem>,
    /// 合集元数据
    pub meta: VideoArchiveMeta,
    /// 分页信息
    pub page: VideoArchivesPage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoArchiveItem {
    pub aid: u64,
    pub bvid: String,
    /// 视频的创建时间戳
    pub ctime: u64,
    /// 视频的时长，单位为秒
    pub duration: u64,
    /// 是否启用虚拟剧场模式
    pub enable_vt: bool,
    /// 是否为互动视频
    pub interactive_video: bool,
    /// 视频封面图片的URL
    pub pic: String,
    /// 视频的播放位置，用于记录用户上次观看的位置
    pub playback_position: u64,
    /// 视频的发布日期
    pub pubdate: u64,
    /// 视频的统计信息
    pub stat: VideoArchiveItemStat,
    /// 视频的标题
    pub title: String,
    /// 用户生成内容的付费类型
    pub ugc_pay: u8,
    /// 虚拟剧场模式的显示信息
    pub vt_display: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoArchiveItemStat {
    /// 观看次数
    pub view: u64,
    pub vt: u64,
}

/// 视频合集元数据结构体
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoArchiveMeta {
    pub category: u64,
    /// 视频封面图片的URL地址
    pub cover: String,
    /// 视频的描述信息
    pub description: String,
    /// up主id
    pub mid: u64,
    /// 合集标题
    pub name: String,
    /// 上传时间戳
    pub ptime: u64,
    /// 合集id
    pub season_id: u64,
    /// 该合集中视频的总数
    pub total: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoArchivesPage {
    /// 页码
    pub page_num: u64,
    /// 每页大小
    pub page_size: u64,
    /// 总数量
    pub total: u64,
}

#[cfg(test)]
mod tests {

    use bili_core::BiliResponse;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_query_video_archive() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = VideoArchiveQuery::builder()
            .mid(296909317)
            .season_id(3091090)
            .build()
            .to_query()
            .unwrap()
            .with_sign(&session.mixin_key())
            .unwrap()
            .to_url(VIDEO_ARCHIVES_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/video/video_archive.json", &json)
            .await
            .unwrap();
    }

    #[test]
    fn test_deserialize_video_archive() {
        let json = include_str!("../../tests/datas/video/video_archive.json");
        serde_json::from_str::<BiliResponse<VideoArchive>>(json).unwrap();
    }
}
