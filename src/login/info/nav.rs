use serde::{Deserialize, Serialize};

use crate::sign::wbi::Wbi;

pub const NAV_URL: &str = "https://api.bilibili.com/x/web-interface/nav";

//导航栏用户信息
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NavData {
    #[serde(rename = "isLogin")]
    pub is_login: bool,
    pub face: String,
    pub mid: u64,
    pub email_verified: u8,
    pub mobile_verified: u8,
    pub level_info: LevelInfo,
    pub money: u64,
    pub moral: i16,
    pub uname: String,
    #[serde(rename = "vipStatus")]
    pub vip_status: u8,
    #[serde(rename = "VipDueDate")]
    pub vip_due_date: u64,
    #[serde(rename = "vipType")]
    pub vip_type: u8,
    pub vip_pay_type: u8,
    pub wbi_img: Wbi,
}
impl NavData {
    pub fn is_login(&self) -> bool {
        self.is_login
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LevelInfo {
    pub current_level: u8,
    pub current_min: u8,
    pub current_exp: u64,
    pub next_exp: String,
}

#[cfg(feature = "session")]
mod session {
    use super::{NavData, NAV_URL};
    use crate::{
        common::{Response, Session},
        error::Error,
        login::info::{InfoData, InfoResponse},
    };

    impl Session {
        pub async fn get_nav(&self) -> Result<NavData, Error> {
            let infodata = self
                .get(NAV_URL)
                .send()
                .await?
                .json::<InfoResponse>()
                .await?
                .result()?;

            if let InfoData::NavData(data) = infodata {
                Ok(data)
            } else {
                Err(Error::from("Unexpected response type"))
            }
        }
    }
}
