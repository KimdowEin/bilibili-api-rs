use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subtitle {
    ///是否允许提交字幕
    pub allow_submit: bool,
    /// 字幕列表
    pub list: Vec<SubtitleItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    // author todo
}
