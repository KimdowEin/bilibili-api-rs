use std::time;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[cfg(feature = "session")]
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SerdeQsError(#[from] serde_qs::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    RsaError1(#[from] rsa::Error),
    #[error(transparent)]
    RsaError2(#[from] rsa::pkcs8::spki::Error),
    #[error(transparent)]
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
