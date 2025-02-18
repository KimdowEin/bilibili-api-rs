use serde::{Deserialize, Serialize};

/// 获取视频合集信息   
/// https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/video/collection.md#%E8%8E%B7%E5%8F%96%E8%A7%86%E9%A2%91%E5%90%88%E9%9B%86%E4%BF%A1%E6%81%AF
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoArchive {
    pub aids: Vec<u64>,
    pub archives: Vec<VideoArchiveItem>,
    pub meta: VideoArchiveMeta,
    pub page: VideoArchivesPage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoArchiveItem {
    pub aid: u64,
    pub bvid: String,
    pub ctime: u64,
    pub duration: u64,
    pub enable_vt: bool,
    pub interactive_video: bool,
    pub pic: String,
    pub playback_position: u64,
    pub pubdate: u64,
    pub stat: VideoArchiveItemStat,
    pub title: String,
    pub ugc_pay: u8,
    pub vt_display: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoArchiveItemStat {
    pub view: u64,
    pub vt: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoArchiveMeta {
    pub category: u64,
    pub cover: String,
    pub description: String,
    pub mid: u64,
    pub name: String,
    pub ptime: u64,
    pub season_id: u64,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoArchivesPage {
    pub page_num: u64,
    pub page_size: u64,
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
