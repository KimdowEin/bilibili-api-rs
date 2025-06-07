// 好友，关注，粉丝等

use bili_core::{Csrf, Query};
use serde::{Deserialize, Serialize};

use crate::model::user::relation::{RelationModifyAction, RelationModifyResource};

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
    pub ps: Option<u32>,
    pub pn: Option<u32>,
}

impl RelationFollowersQuery {
    pub fn new(vmid: u64, ps: Option<u32>, pn: Option<u32>) -> Self {
        Self { vmid, ps, pn }
    }
}
impl From<u64> for RelationFollowersQuery {
    fn from(value: u64) -> Self {
        Self::new(value, None, None)
    }
}

/// 查询用户关注明细
pub const RELATION_FOLLOWINGS_URL: &str = "https://api.bilibili.com/x/relation/followings";

/// 查询用户关注明细
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Query)]
pub struct RelationFollowingsQuery {
    pub vmid: u64,
    pub order_type: Option<String>,
    pub ps: Option<u32>,
    pub pn: Option<u32>,
}
impl RelationFollowingsQuery {
    /// true按最常访问排列，false按关注顺序排列
    pub fn new(vmid: u64, order_type: bool, ps: Option<u32>, pn: Option<u32>) -> Self {
        Self {
            vmid,
            order_type: if order_type {
                Some("attention".to_string())
            } else {
                None
            },
            ps,
            pn,
        }
    }
}
impl From<u64> for RelationFollowingsQuery {
    fn from(vmid: u64) -> Self {
        Self::new(vmid, false, None, None)
    }
}

pub const RELATION_FOLLOWINGS_SEARCH_URL: &str =
    "https://api.bilibili.com/x/relation/followings/search";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Query)]
pub struct RelationFollowingsSearchQuery {
    pub vmid: u64,
    pub name: String,
    pub ps: Option<u32>,
    pub pn: Option<u32>,
}
impl RelationFollowingsSearchQuery {
    pub fn new<S>(vmid: u64, name: S, ps: Option<u32>, pn: Option<u32>) -> Self
    where
        S: Into<String>,
    {
        Self {
            vmid,
            name: name.into(),
            ps,
            pn,
        }
    }
}

/// 查询共同关注明细
pub const RELATION_SAME_FOLLOWINGS_URL: &str =
    "https://api.bilibili.com/x/relation/same/followings";

///查询共同关注明细
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Query)]
pub struct RelationSameFollowingsQuery {
    vmid: u64,
    ps: Option<u32>,
    pn: Option<u32>,
}
impl RelationSameFollowingsQuery {
    pub fn new(vmid: u64, ps: Option<u32>, pn: Option<u32>) -> Self {
        Self { vmid, ps, pn }
    }
}
impl From<u64> for RelationSameFollowingsQuery {
    fn from(value: u64) -> Self {
        Self::new(value, None, None)
    }
}

pub const RELATION_WHISPER_FOLLOWINGS_URL: &str = "https://api.bilibili.com/x/relation/whispers";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Query)]
pub struct RelationWhisperFollowingsQuery;
impl RelationWhisperFollowingsQuery {
    pub fn new() -> Self {
        Self
    }
}

pub const RELATION_FRIENDS_URL: &str = "https://api.bilibili.com/x/relation/friends";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Query)]
pub struct RelationFriendsQuery;
impl RelationFriendsQuery {
    pub fn new() -> Self {
        Self
    }
}

pub const RELATION_BLACKS_URL: &str = "https://api.bilibili.com/x/relation/blacks";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Query)]
pub struct RelationBlacksQuery;
impl RelationBlacksQuery {
    pub fn new() -> Self {
        Self
    }
}

pub const RELATION_MODIFY_URL: &str = "https://api.bilibili.com/x/relation/modify";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Query, Csrf)]
pub struct RelationModifyQuery {
    fid: u64,
    act: RelationModifyAction,
    re_src: RelationModifyResource,
}
impl RelationModifyQuery {
    pub fn new(fid: u64, act: RelationModifyAction, re_src: RelationModifyResource) -> Self {
        Self { fid, act, re_src }
    }
    pub fn modify(fid: u64, act: RelationModifyAction) -> Self {
        Self {
            fid,
            act,
            re_src: RelationModifyResource::UserSpace,
        }
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
            "https://api.bilibili.com/x/relation/followers?vmid=546189"
        )
    }

    #[test]
    fn test_query_relation_followings() {
        let query = RelationFollowingsQuery::from(546189);
        let url = format!("{}?{}", RELATION_FOLLOWINGS_URL, query.to_query().unwrap());
        assert_eq!(
            url,
            "https://api.bilibili.com/x/relation/followings?vmid=546189"
        )
    }

    #[test]
    fn test_query_relation_followings_search() {
        let query = RelationFollowingsSearchQuery::new(293793435, "warma", None, None);
        let url = format!(
            "{}?{}",
            RELATION_FOLLOWINGS_SEARCH_URL,
            query.to_query().unwrap()
        );

        assert_eq!(
            url,
            "https://api.bilibili.com/x/relation/followings/search?vmid=293793435&name=warma"
        )
    }

    #[test]
    fn test_query_relation_same_followings() {
        let query = RelationSameFollowingsQuery::from(546189);
        let url = format!(
            "{}?{}",
            RELATION_SAME_FOLLOWINGS_URL,
            query.to_query().unwrap()
        );

        assert_eq!(
            url,
            "https://api.bilibili.com/x/relation/same/followings?vmid=546189"
        )
    }

    #[test]
    fn test_query_relation_whisper_followings() {
        let query = RelationWhisperFollowingsQuery::new();
        let url = format!(
            "{}?{}",
            RELATION_WHISPER_FOLLOWINGS_URL,
            query.to_query().unwrap()
        );
        assert_eq!(url, "https://api.bilibili.com/x/relation/whispers?")
    }

    #[test]
    fn test_query_relation_friends() {
        let query = RelationFriendsQuery::new();
        let url = format!("{}?{}", RELATION_FRIENDS_URL, query.to_query().unwrap());
        assert_eq!(url, "https://api.bilibili.com/x/relation/friends?")
    }

    #[test]
    fn test_query_relation_blacks() {
        let query = RelationBlacksQuery::new();
        let url = format!("{}?{}", RELATION_BLACKS_URL, query.to_query().unwrap());

        assert_eq!(url, "https://api.bilibili.com/x/relation/blacks?")
    }

    #[test]
    fn test_query_relation_modify() {
        let query = RelationModifyQuery::new(
            1234,
            RelationModifyAction::Follow,
            RelationModifyResource::UserSpace,
        );
        let url = format!("{}?{}", RELATION_MODIFY_URL, query.to_query().unwrap());

        assert_eq!(
            url,
            "https://api.bilibili.com/x/relation/modify?fid=1234&act=1&re_src=11"
        )
    }
}
