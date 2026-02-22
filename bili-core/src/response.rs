//! 响应

use crate::error::Error;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug, Serialize, Deserialize)]
pub struct BiliResponse<D> {
    pub code: i32,
    pub message: String,
    pub data: Option<D>,
}
impl<D> BiliResponse<D> {
    pub fn is_success(&self) -> bool {
        self.code == 0
    }

    /// get data if is success
    pub fn data(self) -> Result<D, Error> {
        if self.is_success() {
            self.data.ok_or(Error::NullResponseDataError)
        } else {
            Err(Error::ResponseError {
                code: self.code,
                msg: self.message,
            })
        }
    }
}

pub trait Data: DeserializeOwned {}
