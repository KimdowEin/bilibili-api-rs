use serde::{Deserialize, Serialize};

use super::state::Dimension;



#[derive(Debug, Serialize, Deserialize)]
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
    /// 封面,如果是view则空
    pub first_frame: Option<String>,
}
