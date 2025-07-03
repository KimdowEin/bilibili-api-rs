//! 核心 traits
//!
//! 分别对应 请求，数据，端口

use crate::{
    auth::AuthType,
    auth::{csrf, sign, to_query},
    error::Error,
};
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "session")]
use crate::service::{bili_request, RequestMethod, Session};
#[cfg(feature = "session")]
use async_trait::async_trait;

pub use bili_derive::*;

pub trait QueryTag: Serialize + Send + Sync {
    const AUTH: AuthType;
}

pub trait Data: DeserializeOwned {}

#[cfg(feature = "session")]
#[async_trait]
pub trait BiliRequest {
    type Query: QueryTag;
    type Response: Data;

    const URL: &str;
    const METHOD: RequestMethod;

    async fn send_request(session: &Session, query: Self::Query) -> Result<Self::Response, Error> {
        let url = match Self::Query::AUTH {
            AuthType::Query => format!("{}?{}", Self::URL, to_query(&query)?),
            AuthType::Sign => format!("{}?{}", Self::URL, sign(&query, &session.bili_jct().await)?),
            AuthType::Csrf => format!("{}?{}", Self::URL, csrf(&query, &session.bili_jct().await)?),
        };
        bili_request(session, url, Self::METHOD).await
    }
}

#[cfg(feature = "session")]
#[macro_export]
macro_rules! use_bili_request {
    () => {
        use crate::{
            define_bili_request, service::session::RequestMethod,
            traits::BiliRequest,
        };
    };
}

#[cfg(feature = "session")]
#[macro_export]
/// Response,url,method,auth
macro_rules! define_bili_request {
    ($response:ident, $url:expr, $method:ident) => {
        paste::paste! {
            // 自动生成结构体名: Response名 + "Request"
            pub struct [<$response Request>];

            impl BiliRequest for [<$response Request>] {
                // 自动生成Query类型: Response名 + "Query"
                type Query = [<$response Query>];
                type Response = $response;

                const URL: &'static str = $url;
                const METHOD: RequestMethod = RequestMethod::$method;
            }
        }
    };
}
