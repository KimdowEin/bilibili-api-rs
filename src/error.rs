use std::time;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[cfg(feature = "session")]
    #[error("{0}")]
    RequestError(#[from] reqwest::Error),

    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    SerdeQsError(#[from] serde_qs::Error),
    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("{0}")]
    RsaError1(#[from] rsa::Error),
    #[error("{0}")]
    RsaError2(#[from] rsa::pkcs8::spki::Error),
    #[error("{0}")]
    SystemTimeError(#[from] time::SystemTimeError),

    #[error("{0}")]
    QueryError(String),
    #[error("{0}")]
    OtherError(String),
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::OtherError(value.to_string())
    }
    
}