//! 密码登录
//!
//! 先完成人机验证拿到validate与seccode
//!
//! 拿到LoginKey,对密码进行加密
//!
//! 登陆(post),检查LoginState判断是否登陆成功

use crate::error::Error;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use bili_core::Data;
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Encrypt, RsaPublicKey};
use serde::{Deserialize, Serialize};

/// 登录盐
///
/// 有效时间为 20s
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct LoginKey {
    #[serde(rename = "hash")]
    pub salt: String,
    pub key: String,
}

/// 密码加密
impl LoginKey {
    pub fn decode_password(&self, password: &str) -> Result<String, Error> {
        let data = format!("{}{}", self.salt, password);

        let mut rng = rand::thread_rng();
        let pub_key = RsaPublicKey::from_public_key_pem(&self.key)?;
        let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, data.as_bytes())?;
        let password = URL_SAFE.encode(enc_data);
        Ok(password)
    }
}

/// 登录响应数据
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct LoginState {
    pub message: String,
    pub refresh_token: String,
    pub timestamp: u64,
    pub url: String,
}
