use crate::session::Session;

impl Session {
    /// 获取视频流地址_web端
    pub async fn web_playurl(&self){
        let url = "https://api.bilibili.com/x/player/wbi/playurl";
    }
}