use crate::traits::Query;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ShareVideoQuery {
    pub aid: Option<u64>,
    pub bvid: Option<String>,
    pub csrf: Option<String>,
}
impl Query for ShareVideoQuery {}
impl ShareVideoQuery {
    pub fn new(aid: Option<u64>, bvid: Option<String>, csrf: Option<String>) -> Self {
        Self { aid, bvid, csrf }
    }
}
