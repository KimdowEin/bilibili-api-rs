use crate::{
    error::Error,
    model::user::relation::{RelationFollowers, RelationFollowings, RelationFollows, RelationStat},
    query::user::relation::{
        RelationBlacksQuery, RelationFollowersQuery, RelationFollowingsQuery,
        RelationFollowingsSearchQuery, RelationFriendsQuery, RelationSameFollowingsQuery,
        RelationStatQuery, RelationWhisperFollowingsQuery, RELATION_FOLLOWERS_URL,
        RELATION_FOLLOWINGS_SEARCH_URL, RELATION_FOLLOWINGS_URL, RELATION_SAME_FOLLOWINGS_URL,
        RELATION_STAT_URL,
    },
    service::{bili_query_get, Session},
};

/// 关系状态数
pub async fn get_relation_stat(
    session: &Session,
    query: RelationStatQuery,
) -> Result<RelationStat, Error> {
    bili_query_get(session, RELATION_STAT_URL, query).await
}

/// 查询用户粉丝明细
pub async fn get_relation_followers(
    session: &Session,
    query: RelationFollowersQuery,
) -> Result<RelationFollowers, Error> {
    bili_query_get(session, RELATION_FOLLOWERS_URL, query).await
}

/// 查询用户关注明细
pub async fn get_relation_followings(
    session: &Session,
    query: RelationFollowingsQuery,
) -> Result<RelationFollowings, Error> {
    bili_query_get(session, RELATION_FOLLOWINGS_URL, query).await
}

pub async fn get_relation_followings_search(
    session: &Session,
    query: RelationFollowingsSearchQuery,
) -> Result<RelationFollowings, Error> {
    bili_query_get(session, RELATION_FOLLOWINGS_SEARCH_URL, query).await
}

pub async fn get_realtion_same_followings(
    session: &Session,
    query: RelationSameFollowingsQuery,
) -> Result<RelationFollowings, Error> {
    bili_query_get(session, RELATION_SAME_FOLLOWINGS_URL, query).await
}

pub async fn get_relation_whisper_followings(
    session: &Session,
    query: RelationWhisperFollowingsQuery,
) -> Result<RelationFollowings, Error> {
    bili_query_get(session, RELATION_FOLLOWINGS_URL, query).await
}

pub async fn get_relation_friends(
    session: &Session,
    query: RelationFriendsQuery,
) -> Result<RelationFollows, Error> {
    bili_query_get(session, RELATION_FOLLOWINGS_URL, query).await
}

pub async fn get_relation_blacks(
    session: &Session,
    query: RelationBlacksQuery,
) -> Result<RelationFollows, Error> {
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
