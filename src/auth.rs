//! 请求鉴权

use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::error::Error;

pub enum AuthType {
    /// 无鉴权
    Query,

    Sign,
    Csrf,
}

pub fn to_query(query: &impl Serialize) -> Result<String, Error> {
    Ok(serde_qs::to_string(query)?)
}
pub fn sign(query: &impl Serialize, mixin_key: &str) -> Result<String, Error> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let wts = format!("wts={}", timestamp);

    let ori_query = to_query(query)?;
    let mut querys = ori_query.split("&").collect::<Vec<&str>>();
    querys.push(&wts);
    querys.sort();
    let mut hash_query = querys.join("&");
    hash_query.push_str(mixin_key);

    let w_rid = format!("w_rid={:?}", md5::compute(&hash_query));
    let query = format!("{}&{}&{}", ori_query, w_rid, wts);
    Ok(query)
}

pub fn csrf(query: &impl Serialize, bili_jct: &str) -> Result<String, Error> {
    let ori_query = to_query(query)?;
    let query = format!("{}&csrf={}", ori_query, bili_jct);
    Ok(query)
}
