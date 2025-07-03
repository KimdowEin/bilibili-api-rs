use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_bool_from_anything;

use crate::traits::Data;

/// 实际无返回，为了配合bilirequest trait而定义的LikeVideo类型
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Data)]
pub struct LikeVideo(pub bool);
impl Deref for LikeVideo {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for LikeVideo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Data)]
pub struct IsLikeVideo(
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub bool
);
impl Deref for IsLikeVideo {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for IsLikeVideo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
