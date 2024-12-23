use serde::{Deserialize, Serialize};
use crate::traits::Query;


#[derive(Serialize, Deserialize, Debug,Default, Clone)]
pub struct ShareVideoQuery{
    aid: Option<u64>,
    bvid: Option<String>,
    csrf:Option<String>,
}
impl Query for ShareVideoQuery{
}
impl ShareVideoQuery {
    pub fn new(aid: Option<u64>, bvid: Option<String>, csrf: Option<String>) -> Self {
        Self {
            aid,
            bvid,
            csrf,
        }
    }
}