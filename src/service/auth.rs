//! 签名操作

use_bili_request!();
use crate::{
    error::Error,
    model::auth::ticket::BiliTicket,
    query::auth::ticket::{BiliTicketQuery, BILI_TICKET_URL},
    service::Session,
    use_bili_request,
};

use super::session::COOKIES_URL;

define_bili_request!(BiliTicket, BILI_TICKET_URL, Post);

impl Session {
    /// 刷新 获得ticket 获得wbi key 从cookies获取csrf(bili_jct)
    pub async fn refresh_auth(&self) -> Result<(), Error> {
        self.refresh_csrf().await?;

        let query = BiliTicketQuery::new()?;
        let ticket = BiliTicketRequest::send_request(&self, query).await?;

        self.set_ticket(&ticket.ticket);

        let mixin_key = ticket.wbi.mixin_key();
        self.set_mixin_key(&mixin_key).await;

        Ok(())
    }

    pub async fn refresh_csrf(&self) -> Result<(), Error> {
        if let Some(bili_jct) = self.get_cookie(COOKIES_URL, "bili_jct") {
            self.set_bili_jct(&bili_jct).await;
            Ok(())
        } else {
            Err(Error::OtherError("未登录".to_string()))?
        }
    }

    /// 获取 wbi 签名，每日更新
    pub async fn get_mixin_key(&self) -> Result<(), Error> {
        let wbi = self.get_nav().await?.wbi_img;
        let mixin_key = wbi.mixin_key();
        self.set_mixin_key(&mixin_key).await;
        Ok(())
    }

    /// 设置 wbi 签名
    pub async fn set_mixin_key(&self, mixin_key: &str) {
        let mut m = self.mixin_key.write().await;
        *m = mixin_key.to_string();
    }
    /// 访问 wbi key
    pub async fn mixin_key(&self) -> String {
        self.mixin_key.read().await.clone()
    }

    /// 设置bili_jct
    pub async fn set_bili_jct(&self, bili_jct: &str) {
        let mut m = self.bili_jct.write().await;
        *m = bili_jct.to_string();
    }

    /// 访问csrf(bili_jct)
    pub async fn bili_jct(&self) -> String {
        self.bili_jct.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_refresh_sign() {
        let session = Session::new_with_path("cookies.json").unwrap();
        session.refresh_auth().await.unwrap();

        // session.get_nav().await.unwrap();

        session.save_cookies().unwrap();
    }
}
