use crate::{
    error::Error,
    model::user::nav::NavInfo,
    query::user::nav::{NavInfoQuery, NAV_INFO_URL},
    service::session::Session,
    use_bili_request,
};

use_bili_request!();
define_bili_request!(NavInfo, NAV_INFO_URL, Get);

impl Session {
    pub async fn get_nav(&self) -> Result<NavInfo, Error> {
        let query = NavInfoQuery::new();
        NavInfoRequest::send_request(&self, query).await
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
