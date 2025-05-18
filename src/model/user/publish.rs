use bili_core::Data;
use serde::{Deserialize, Serialize};




#[derive(Debug,Clone,PartialEq, Deserialize, Serialize,Data)]
pub struct PublishUpStat{
    /// 视频播放量
    pub archive:PublishUpStatArchive,
    /// 专栏阅读量	
    pub article:PublishUpStatArticle,
    pub likes:u64,
}

#[derive(Debug,Clone,PartialEq, Deserialize, Serialize)]
pub struct PublishUpStatArchive{
    pub view:u64,
}

#[derive(Debug,Clone,PartialEq, Deserialize, Serialize)]
pub struct PublishUpStatArticle{
    pub view:u64,
}