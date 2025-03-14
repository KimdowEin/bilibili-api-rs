use crate::{
    error::Error,
    model::{response::BiliResponse, sign::ticket::BiliTicket},
    query::sign::ticket::{BiliTicketQuery, BILI_TICKET_URL},
    traits::Csrf,
};

use super::session::{Session, COOKIES_URL};

impl Session {
    /// 获得ticket
    pub async fn get_ticket(&self, query: BiliTicketQuery) -> Result<BiliTicket, Error> {
        let url = format!("{}?{}", BILI_TICKET_URL, query.csrf(&self.bili_jct().await)?);
        self.post(url)
            .send()
            .await?
            .json::<BiliResponse<_>>()
            .await?
            .data()
    }

    /// 刷新 获得ticket 获得wbi key 从cookies获取csrf(bili_jct)
    pub async fn refresh_sign(&self) -> Result<(), Error> {
        if let Some(bili_jct) = self.get_cookie(COOKIES_URL, "bili_jct") {
            self.set_bili_jct(&bili_jct).await;
        } else {
            Err(Error::OtherError("未登录".to_string()))?
        };

        let query = BiliTicketQuery::new()?;
        let ticket = self.get_ticket(query).await?;

        self.set_ticket(&ticket.ticket);

        let mixin_key = ticket.wbi.mixin_key();
        self.set_mixin_key(&mixin_key).await;

        Ok(())
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
    pub async fn set_bili_jct(&self,bili_jct:&str){
        let mut m = self.bili_jct.write().await;
        *m = bili_jct.to_string();
    }
    
    /// 访问csrf(bili_jct)
    pub async fn bili_jct(&self)->String{
        self.bili_jct.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_refresh_sign() {
        let session = Session::new_with_path("cookies.json").unwrap();
        session.refresh_sign().await.unwrap();

        session.get_nav().await.unwrap();

        session.save_cookies().unwrap();
    }
}
