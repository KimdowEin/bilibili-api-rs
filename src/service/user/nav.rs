use crate::{
    error::Error,
    model::{response::BiliResponse, user::nav::NavInfo},
    query::user::nav::NAV_INFO_URL,
    service::session::Session,
};

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

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_nav() {
        let session = Session::new_with_path("./cookies.json").unwrap();
        let nav = session.get_nav().await.unwrap();
        assert!(nav.is_login);
        assert!(nav.nav.is_some());
    }
}
