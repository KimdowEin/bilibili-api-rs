use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectVideo {
    pub prompt: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsCollect {
    pub favoured: bool,
}
