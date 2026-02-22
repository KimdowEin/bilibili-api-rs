use std::time::{SystemTime, UNIX_EPOCH};

use derive_more::{AsRef, Deref, DerefMut, Display, From, Into};
use serde::Serialize;

use crate::error::Error;

#[derive(Debug, Clone, AsRef, Deref, DerefMut, From, Into, Display)]
pub struct Query(String);
impl Query {
    pub fn with_sign(&self, mixin_key: &str) -> Result<Query, Error> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let wts = format!("wts={}", timestamp);

        let mut querys = self.split("&").collect::<Vec<&str>>();
        querys.push(&wts);
        querys.sort();
        let mut hash_query = querys.join("&");
        hash_query.push_str(mixin_key);

        let w_rid = format!("w_rid={:x}", md5::compute(&hash_query));
        let query = format!("{}&{}&{}", self, w_rid, wts).into();
        Ok(query)
    }

    pub fn with_csrf(&self, bili_jct: &str) -> Result<Query, Error> {
        let query = format!("{}&csrf={}", self, bili_jct).into();
        Ok(query)
    }

    pub fn to_url(self, bash: &str) -> String {
        format!("{}?{}", bash, self)
    }
    pub fn inner(self) -> String {
        self.0
    }
}

pub trait ToQuery: Serialize {
    fn to_query(&self) -> Result<Query, Error> {
        let query_str = serde_qs::to_string(&self)?.into();
        Ok(query_str)
    }
}

pub use bili_derive::*;
