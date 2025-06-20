//! 核心 traits
//!
//! 分别对应 请求，数据，端口

use crate::{
    auth::{csrf, sign, AuthType},
    error::Error,
};

#[cfg(feature = "session")]
use crate::service::{bili_request, RequestMethod, Session};
#[cfg(feature = "session")]
use async_trait::async_trait;

pub use bili_derive::*;

use serde::{de::DeserializeOwned, Serialize};

pub trait Query: Serialize + Sized + Send + Sync {
    /// 生成原始query
    fn to_query(&self) -> Result<String, Error> {
        Ok(serde_qs::to_string(self)?)
    }
}

pub trait Sign: Query {
    /// 生成需要签名的query
    fn sign(&self, mixin_key: &str) -> Result<String, Error> {
        sign(self, mixin_key)
    }
}

pub trait Csrf: Query {
    fn csrf(&self, bili_jct: &str) -> Result<String, Error> {
        csrf(self, bili_jct)
    }
}

pub trait Data: DeserializeOwned + Sized {}

#[cfg(feature = "session")]
#[async_trait]
pub trait BiliRequest {
    type Query: Query;
    type Response: Data;

    const URL: &str;
    const METHOD: RequestMethod;
    const AUTH: AuthType;

    async fn send_request(session: &Session, query: Self::Query) -> Result<Self::Response, Error> {
        let url = match Self::AUTH {
            AuthType::None => format!("{}?{}", Self::URL, query.to_query()?),
            AuthType::Sign => format!("{}?{}", Self::URL, sign(&query, &session.bili_jct().await)?),
            AuthType::Csrf => format!("{}?{}", Self::URL, csrf(&query, &session.bili_jct().await)?),
        };
        bili_request(session, url, Self::METHOD).await
    }
}
