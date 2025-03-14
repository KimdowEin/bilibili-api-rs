use crate::{
    error::Error,
    query::video::VideoQuery,
    traits::{Csrf, Query},
};


pub const SHARE_VIDEO_URL: &str = "https://api.bilibili.com/x/web-interface/share/add";

pub type ShareVideoQuery = VideoQuery;

impl Csrf for ShareVideoQuery {
    fn csrf(&self, bili_jct: &str) -> Result<String, Error> {
        let ori_query = self.to_query()?;
        if bili_jct.is_empty() {
            Ok(ori_query)
        } else {
            let query = format!("{}&csrf={}", ori_query, bili_jct);
            Ok(query)
        }
    }
}
