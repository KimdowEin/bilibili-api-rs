use crate::{
    model::user::relation::{
        RelationBlacks, RelationFollowers, RelationFollowings, RelationFollowingsSearch,
        RelationFriends, RelationSameFollowings, RelationStat, RelationWhisperFollowings,
    },
    query::user::relation::{
        RelationBlacksQuery, RelationFollowersQuery, RelationFollowingsQuery,
        RelationFollowingsSearchQuery, RelationFriendsQuery, RelationSameFollowingsQuery,
        RelationStatQuery, RelationWhisperFollowingsQuery, RELATION_BLACKS_URL,
        RELATION_FOLLOWERS_URL, RELATION_FOLLOWINGS_SEARCH_URL, RELATION_FOLLOWINGS_URL,
        RELATION_FRIENDS_URL, RELATION_SAME_FOLLOWINGS_URL, RELATION_STAT_URL,
        RELATION_WHISPER_FOLLOWINGS_URL,
    },
    use_bili_request,
};

use_bili_request!();

define_bili_request!(RelationStat, RELATION_STAT_URL, Get);
define_bili_request!(RelationFollowers, RELATION_FOLLOWERS_URL, Get);
define_bili_request!(RelationFollowings, RELATION_FOLLOWINGS_URL, Get);
define_bili_request!(
    RelationFollowingsSearch,
    RELATION_FOLLOWINGS_SEARCH_URL,
    Get
);
define_bili_request!(RelationSameFollowings, RELATION_SAME_FOLLOWINGS_URL, Get);
define_bili_request!(
    RelationWhisperFollowings,
    RELATION_WHISPER_FOLLOWINGS_URL,
    Get
);
define_bili_request!(RelationFriends, RELATION_FRIENDS_URL, Get);
define_bili_request!(RelationBlacks, RELATION_BLACKS_URL, Get);

#[cfg(test)]
mod tests {
    use crate::service::Session;

    use super::*;

    #[tokio::test]
    async fn test_get_relation_stat() {
        let session = Session::from("./cookies.json");

        let query = RelationStatQuery::new(200435669);
        let stat = RelationStatRequest::send_request(&session, query)
            .await
            .unwrap();

        assert_eq!(stat.mid, 200435669);
    }

    #[tokio::test]
    async fn test_get_relation_followers() {
        let session = Session::from("./cookies.json");

        let query = RelationFollowersQuery::from(200435669);
        let _ = RelationFollowersRequest::send_request(&session, query)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_relation_followings() {
        let session = Session::from("./cookies.json");

        let query = RelationFollowingsQuery::from(200435669);
        let _ = RelationFollowingsRequest::send_request(&session, query)
            .await
            .unwrap();
    }
}
