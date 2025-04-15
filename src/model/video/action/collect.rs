use serde::{Deserialize, Serialize};
use crate::Data;

#[derive(Debug, Clone, PartialEq,Eq, Serialize, Deserialize,Data)]
pub struct CollectVideo {
    pub prompt: bool,
}

#[derive(Debug, Clone, PartialEq,Eq, Serialize, Deserialize,Data)]
pub struct IsCollect {
    pub favoured: bool,
}
