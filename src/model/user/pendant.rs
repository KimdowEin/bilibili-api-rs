use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pendant {
    pub pid: u64,
    pub name: String,
    pub image: String,
    pub image_enhance: String,
    pub image_enhance_frame: String,
    pub n_pid: u64,
}
