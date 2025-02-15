use serde::{Deserialize, Serialize};

use super::state::Dimension;



#[derive(Debug,Clone,PartialEq, Serialize, Deserialize)]
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
    // /// 站外视频vid
    pub vid: String,
    // /// 站外视频跳转链接
    pub weblink: String,
    /// 分P分辨率
    pub dimension: Dimension,
    
    // /// 封面,如果是view则空
    // pub first_frame: Option<String>,
}



#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_deserize_cid(){
        let json = r#"
            [
                {
                    "cid": 993004511,
                    "page": 1,
                    "from": "vupload",
                    "part": "あるふぁ_可不",
                    "duration": 206,
                    "vid": "",
                    "weblink": "",
                    "dimension": {
                        "width": 1920,
                        "height": 1080,
                        "rotate": 0
                    },
                    "first_frame": "http://i1.hdslb.com/bfs/storyff/n230204a2sbv2kc5ydvkx3j069df5uco_firsti.jpg"
                },
                {
                    "cid": 993074945,
                    "page": 2,
                    "from": "vupload",
                    "part": "カンザキイオリ short ver.",
                    "duration": 41,
                    "vid": "",
                    "weblink": "",
                    "dimension": {
                        "width": 1280,
                        "height": 720,
                        "rotate": 0
                    },
                    "first_frame": "http://i1.hdslb.com/bfs/storyff/n230204a23v9nzyfoy1gwz1p4ej2x9t0_firsti.jpg"
                }
            ]
        "#;
        serde_json::from_str::<Vec<Cids>>(json).unwrap();

    }
}