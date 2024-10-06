pub mod video_info;
pub mod video_info_detail;
pub mod archive_desc;
pub mod player_pagelist;






#[cfg(feature = "session")]

mod session {
    use super::*;
    use crate::common::Session;
    use crate::common::{Data, ResponseData};
    use reqwest::Error;

    impl Session {
        pub async fn get_web_video_info(&self, query: String) -> Result<WebVideoInfoData, Error> {
            let url = format!("{}?{}", WEB_VIDEO_INFO_URL, query);
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
}
#[cfg(feature = "session")]
pub use session::*;
