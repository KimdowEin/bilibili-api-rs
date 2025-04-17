use serde::{Deserialize, Serialize};

use crate::Data;

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
    use super::*;

    #[test]
    fn test_deserilize_video_archive() {
        let json = r#"{
            "aids": [
                113868625940323,
                113828712941670
            ],
            "archives": [
                {
                    "aid": 113868625940323,
                    "bvid": "BV1V7f4YpEG1",
                    "ctime": 1737498173,
                    "duration": 777,
                    "enable_vt": false,
                    "interactive_video": false,
                    "pic": "http://i2.hdslb.com/bfs/archive/930eb3ec7009086d5c36ca0bbd55aacbe24868b5.jpg",
                    "playback_position": 0,
                    "pubdate": 1737543600,
                    "stat": {
                        "view": 1796,
                        "vt": 0
                    },
                    "state": 0,
                    "title": "【姬雏放送部】禁止片假名的传话游戏玩到降智ｗｗｗ",
                    "ugc_pay": 0,
                    "vt_display": ""
                },
                {
                    "aid": 113828712941670,
                    "bvid": "BV191cYe5ExJ",
                    "ctime": 1736889216,
                    "duration": 450,
                    "enable_vt": false,
                    "interactive_video": false,
                    "pic": "http://i0.hdslb.com/bfs/archive/c9d411b2fb5e9fdc9a9b4d6ce643a364d8a8e302.jpg",
                    "playback_position": 0,
                    "pubdate": 1736938800,
                    "stat": {
                        "view": 4156,
                        "vt": 0
                    },
                    "state": 0,
                    "title": "新Club 田中都是惠 第六夜『将来的选择』",
                    "ugc_pay": 0,
                    "vt_display": ""
                }
            ],
            "meta": {
                "category": 0,
                "cover": "https://archive.biliimg.com/bfs/archive/05d990e5a9123d9b32ee135a894f82119907b3a5.jpg",
                "description": "",
                "mid": 296909317,
                "name": "合集·[会限]充电专属视频",
                "ptime": 1737543600,
                "season_id": 3091090,
                "total": 67
            },
            "page": {
                "page_num": 1,
                "page_size": 30,
                "total": 67
            }
        }"#;
        serde_json::from_str::<VideoArchive>(json).unwrap();
    }
}
