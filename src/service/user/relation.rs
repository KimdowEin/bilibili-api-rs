use crate::{error::Error, model::user::relation::{RelationFollowers, RelationFollowings, RelationStat}, query::user::relation::{RelationFollowersQuery, RelationFollowingsQuery, RelationStatQuery, RELATION_FOLLOWERS_URL, RELATION_FOLLOWINGS_URL, RELATION_STAT_URL}, service::{bili_get, bili_query_get, Session}};



/// 关系状态数
pub async fn get_relation_stat(session:&Session,query:RelationStatQuery)->Result<RelationStat, Error>{
    bili_query_get(session, RELATION_STAT_URL, query).await
}

/// 查询用户粉丝明细
pub async fn get_relation_followers(session:&Session,query:RelationFollowersQuery)->Result<RelationFollowers, Error>{
    bili_query_get(session, RELATION_FOLLOWERS_URL, query).await
}

/// 查询用户关注明细
pub async fn get_relation_followings(session:&Session,query:RelationFollowingsQuery)->Result<RelationFollowings, Error>{
    bili_query_get(session, RELATION_FOLLOWINGS_URL, query).await
}








#[cfg(test)]
mod tests { 
    use super::*;

    #[tokio::test]
    async fn test_get_relation_stat() {
        let session = Session::from("./cookies.json");

        let query = RelationStatQuery::new(200435669);
        let stat = get_relation_stat(&session, query).await.unwrap();

        assert_eq!(stat.mid, 200435669);
    }
    
    #[tokio::test]
    async fn test_get_relation_followers() {
        let session = Session::from("./cookies.json");

        let query = RelationFollowersQuery::from(200435669);
        let _ = get_relation_followers(&session, query).await.unwrap();

    }

    #[tokio::test]
    async fn test_get_relation_followings() {
        let session = Session::from("./cookies.json");

        let query = RelationFollowingsQuery::from(200435669);
        let _ = get_relation_followings(&session, query).await.unwrap();
    }
}