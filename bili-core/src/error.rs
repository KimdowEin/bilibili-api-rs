use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    QsError(#[from] serde_qs::Error),
    #[error("the response has not data,but it may be success")]
    NullResponseDataError,
    #[error("err code:{code},{msg}")]
    ResponseError { code: i32, msg: String },
}
