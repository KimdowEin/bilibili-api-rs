use crate::error::Error;

use super::session::Session;

impl Session {
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
    pub async fn key(&self) -> String {
        self.mixin_key.read().await.clone()
    }
}
