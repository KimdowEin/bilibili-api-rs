use crate::model::response::BiliResponseCode;
use std::time;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[cfg(feature = "session")]
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error("bilibili response error, code:{code:?}, message:{message}")]
    ResponseError {
        code: BiliResponseCode,
        message: String,
    },

    #[error("bilibili response data is null,code:{0:?}")]
    NullResponseError(BiliResponseCode),

    #[error(transparent)]
    QsError(#[from] serde_qs::Error),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    SignError(#[from] rsa::pkcs8::spki::Error),
    #[error(transparent)]
    SignError2(#[from] rsa::errors::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    TimeError(#[from] time::SystemTimeError),

    #[error("other error:{0}")]
    OtherError(String),
}
