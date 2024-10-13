use std::{error::Error as Err, fmt::Display};

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "session")]
    RequsetError(reqwest::Error),
    IoError(std::io::Error),
    ResponseError(ResponseError),
    SerdeQsError(serde_qs::Error),
    SerdeJsonError(serde_json::Error),
    OtherError(String),
}

#[derive(Debug)]
pub struct ResponseError {
    message: String,
}
impl Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl From<String> for ResponseError {
    fn from(value: String) -> Self {
        ResponseError { message: value }
    }
}
// impl<R> From<R> for ResponseError
// where R:Response{
//     fn from(value: R) -> Self {
//         ResponseError {
//             message: value.message()
//         }
//     }
// }

#[cfg(feature = "session")]
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::RequsetError(value)
    }
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}
impl From<serde_qs::Error> for Error {
    fn from(value: serde_qs::Error) -> Self {
        Error::SerdeQsError(value)
    }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::SerdeJsonError(value)
    }
}

impl From<ResponseError> for Error {
    fn from(value: ResponseError) -> Self {
        Error::ResponseError(value)
    }
}
impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::OtherError(value.to_string())
    }
}
impl Err for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "session")]
            Error::RequsetError(e) => write!(f, "RequsetError: {}", e),
            Error::IoError(e) => write!(f, "IoError: {}", e),
            Error::ResponseError(e) => write!(f, "ResponseError: {}", e),
            Error::OtherError(e) => write!(f, "OtherError: {}", e),
            Error::SerdeQsError(e) => write!(f, "SerdeQsError: {}", e),
            Error::SerdeJsonError(e) => write!(f, "SerdeJsonError: {}", e),
        }
    }
}
