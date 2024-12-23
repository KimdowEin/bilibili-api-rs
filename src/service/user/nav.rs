use crate::{error::Error, model::{response::BiliResponse, user::nav::NavInfo}, query::user::nav::NAV_INFO_URL, service::session::Session};

impl Session {
    pub async fn get_nav(&self) -> Result<NavInfo, Error> {
        self.get(NAV_INFO_URL)
            .send()
            .await?
            .json::<BiliResponse<_>>()
            .await?
            .data()
    }
}