// 好友，关注，粉丝等

use bili_core::Query;
use serde::{Deserialize, Serialize};

/// 关系状态数
pub const RELATION_STAT_URL: &str = "https://api.bilibili.com/x/relation/stat";

/// 关系状态数
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Query)]
pub struct RelationStatQuery {
    pub vmid: u64,
}
impl RelationStatQuery {
    pub fn new(vmid: u64) -> Self {
        Self { vmid }
    }
}
impl From<u64> for RelationStatQuery {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

/// 查询用户粉丝明细
pub const RELATION_FOLLOWERS_URL: &str = "https://api.bilibili.com/x/relation/followers";

/// 查询用户粉丝明细
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Query)]
pub struct RelationFollowersQuery {
    pub vmid: u64,
    pub ps: u64,
    pub pn: u64,
}

impl RelationFollowersQuery {
    pub fn new(vmid: u64, ps: Option<u64>, pn: Option<u64>) -> Self {
        let ps = ps.unwrap_or(50);
        let pn = pn.unwrap_or(1);

        Self { vmid, ps, pn }
    }
}
impl From<u64> for RelationFollowersQuery {
    fn from(value: u64) -> Self {
        Self::new(value, None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_relation_stat() {
        let query = RelationStatQuery::from(200435669);
        let url = format!("{}?{}", RELATION_STAT_URL, query.to_query().unwrap());
        assert_eq!(
            url,
            "https://api.bilibili.com/x/relation/stat?vmid=200435669"
        )
    }

    #[test]
    fn test_query_relation_followers() {
        let query = RelationFollowersQuery::from(546189);
        let url = format!("{}?{}", RELATION_FOLLOWERS_URL, query.to_query().unwrap());
        assert_eq!(
            url,
            "https://api.bilibili.com/x/relation/followers?vmid=546189&ps=50&pn=1"
        )
  }
}
