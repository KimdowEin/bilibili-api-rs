use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SpkiError(#[from] rsa::pkcs8::spki::Error),
    #[error(transparent)]
    RsaError(#[from] rsa::Error),
}
