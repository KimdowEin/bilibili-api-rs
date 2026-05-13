use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamePlate {
    pub nid: i32,
    pub name: String,
    pub image: String,
    pub image_small: String,
    pub level: String,
    pub condition: String,
}
