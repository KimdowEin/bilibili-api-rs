use crate::{error::Error, model::{response::BiliResponse, sign::ticket::BiliTicket}, query::sign::ticket::{BiliTicketQuery, BILI_TICKET_URL}, traits::Query};

use super::session::{Session, COOKIES_URL};

impl Session {
    pub async fn get_ticket(&self,query: BiliTicketQuery) -> Result<BiliTicket, Error> {
        let url = format!("{}?{}",BILI_TICKET_URL, query.to_query()?);
        self.post(url)
            .send()
            .await?
            .json::<BiliResponse<_>>()
            .await?
            .data()
    }
    pub async fn refresh_sign(&mut self) -> Result<(), Error> {
        let csrf = if let Some(csrf) = self.get_cookie(COOKIES_URL, "bili_jct") {
            csrf
        } else {
            Err(Error::OtherError("未登录".to_string()))?
        };

        let query = BiliTicketQuery::new(csrf)?;
        let ticket = self.get_ticket(query).await?;
        
        self.set_ticket(&ticket.ticket);
        self.set_mixin_key(ticket.wbi.mixin_key()).await;

        Ok(())
    }

    /// 获取 wbi 签名，每日更新
    pub async fn get_mixin_key(&mut self) -> Result<(), Error> {
        let wbi = self.get_nav().await?.wbi_img;
        let mixin_key = wbi.mixin_key();
        self.set_mixin_key(mixin_key).await;
        Ok(())
    }

    /// 设置 wbi 签名
    pub async fn set_mixin_key(&mut self, mixin_key: String) {
        let mut m = self.mixin_key.write().await;
        *m = mixin_key;
    }
    /// 获取 wbi key
    pub async fn mixin_key(&self) -> String {
        self.mixin_key.read().await.clone()
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[tokio::test]
    async fn test_refresh_sign(){
        let mut session = Session::new_with_path("cookies.json").unwrap();
        session.refresh_sign().await.unwrap();

        session.get_nav().await.unwrap();

        session.save_cookies().unwrap();
    }
}
