use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct FollowStat {
    // 关注
    pub following: u64,
    // 粉丝
    pub follower: u64,
    // 动态
    pub dynamic_count: u64,
}

