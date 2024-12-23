use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[cfg(feature = "session")]
    #[error("ClientBuildError: {0}")]
    ClientBuildError(#[from] reqwest::Error),

    #[error("RequsetError: {0}")]
    RequsetError(String),
    #[error("ResponseError: {0}")]
    ResponseError(String),
    #[error("IOError: {0}")]
    IoError(#[from] std::io::Error),
    #[error("SerdeQsError: {0}")]
    SerdeQsError(#[from] serde_qs::Error),
    #[error("SerdeJsonError: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("RsaError: {0}")]
    RsaError1(#[from] rsa::Error),
    #[error("RsaError: {0}")]
    RsaError2(#[from] rsa::pkcs8::spki::Error),
    #[error("OtherError: {0}")]
    OtherError(String),
}
