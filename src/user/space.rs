use contribute::VideoContributeData;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub mod contribute;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpaceResponse {
    code: SpaceResponseCode,
    message: String,
    data: SpaceData,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum SpaceResponseCode {
    Success = 0,
    RequestError = -400,
    RequestForbidden = -412,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpaceData {
    VideoContributeData(VideoContributeData),
}
