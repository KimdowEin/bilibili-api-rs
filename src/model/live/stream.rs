use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Quality {
    FL = 2,
    HD = 3,
    ART = 4,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum LiveStreamQn {
    FL = 80,
    HD = 150,
    FHD = 250,
    Blue = 400,
    ART = 10000,
    K4 = 20000,
    Dolby = 30000,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStream {
    pub current_quality: Quality,
    pub accept_quality: Vec<String>,
    pub current_qn: Quality,
    pub quality_description: Vec<QualityAndDesc>,
    pub durl: Vec<Durl>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAndDesc {
    qn: Quality,
    desc: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Durl {
    pub url: String,
    pub order: i64,
}
