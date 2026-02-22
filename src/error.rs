use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    BiliCoreError(#[from] bili_core::error::Error),

    #[error(transparent)]
    EncryptError(#[from] bili_login::error::Error),

    #[error(transparent)]
    ServiceError(#[from] bili_service::error::Error),
}
