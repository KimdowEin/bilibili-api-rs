//! 视频分P

use crate::Data;
use serde::{Deserialize, Serialize};

use super::state::Dimension;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct Cids {
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
    /// 站外视频vid
    pub vid: String,
    /// 站外视频跳转链接
    pub weblink: String,
    /// 分P分辨率
    pub dimension: Dimension,
    /// 封面,如果是VideoView则空
    pub first_frame: Option<String>,
    /// 上传时间
    pub ctime: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deserize_cid() {
        let json = r#"[
            {
                "cid": 29193274957,
                "page": 1,
                "from": "vupload",
                "part": "DECO*27 - 弱虫モンブラン (Reloaded) feat. 初音ミク",
                "duration": 233,
                "vid": "",
                "weblink": "",
                "dimension": {
                    "width": 1920,
                    "height": 1080,
                    "rotate": 0
                },
                "first_frame": "http://i1.hdslb.com/bfs/storyff/n250402ad7b92buy6pu3l1fkdd2ctydl_firsti.jpg",
                "ctime": 1743565693
            }
        ]"#;
        serde_json::from_str::<Vec<Cids>>(json).unwrap();
    }
}
