use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ParseError(#[from] url::ParseError),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    CookieError(#[from] cookie_store::CookieError),
    #[error(transparent)]
    CookieStoreError(#[from] cookie_store::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
