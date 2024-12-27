use std::time::{SystemTime, UNIX_EPOCH};

use serde::{de::DeserializeOwned, Serialize};

use crate::error::Error;


pub trait WbiSign {}

pub trait Query: Serialize+ DeserializeOwned + Sized {

    /// 生成原始query
    fn to_query(&self) -> Result<String, Error> {
        Ok(serde_qs::to_string(self)?)
    }

    /// 生成需要签名的query
    fn sign(&self, mixin_key: &str) -> Result<String, Error>
    where
        Self: WbiSign,
    {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let wts = format!("wts={}", timestamp);
        let ori_query = self.to_query()?;
        let mut querys = ori_query.split("&").collect::<Vec<&str>>();
        querys.push(&wts);
        querys.sort();
        let mut hash_query = querys.join("&");
        hash_query.push_str(mixin_key);

        let w_rid = format!("w_rid={:?}", md5::compute(&hash_query));
        let query = format!("{}&{}&{}", ori_query, w_rid, wts);
        Ok(query)
    }

}