use crate::{
    model::user::publish::PublishUpStat,
    query::user::publish::{PublishUpStatQuery, PUBLISH_UP_STAT_URL},
    use_bili_request,
};

use_bili_request!();

define_bili_request!(PublishUpStat, PUBLISH_UP_STAT_URL, Get);
