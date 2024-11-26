use std::{error::Error as Err, fmt::Display};


use thiserror::Error;

#[derive(Error,Debug)]
pub enum Error {
    #[error("RequsetError: {0}")]
    ResponseError(String),
    #[error("IOError: {0}")]
    IoError(#[from] std::io::Error),
    #[error("SerdeQsError: {0}")]
    SerdeQsError(#[from]serde_qs::Error),
    #[error("SerdeJsonError: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("RsaError: {0}")]
    RsaError1(#[from] rsa::Error),
    #[error("RsaError: {0}")]
    RsaError2(#[from] rsa::pkcs8::spki::Error),
    #[error("OtherError: {0}")]
    OtherError(String),
    // #[cfg(feature = "session")]
    // RequsetError(reqwest::Error),
}

#[cfg(feature = "session")]
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::RequsetError(value)
    }
}

