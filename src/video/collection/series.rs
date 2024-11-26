use serde::{Deserialize, Serialize};

use crate::common::{Query, WbiSign};

use super::{CollectArchi, CollectionPage};

pub const SEASONS_SERIES_URL: &str =
    "https://api.bilibili.com/x/polymer/web-space/home/seasons_series";

#[derive(Debug, Serialize, Deserialize)]
pub struct SeriesQuery {
    mid: u64,
    page_num: u64,
    page_size: u64,
}
impl Query for SeriesQuery {}
impl WbiSign for SeriesQuery {}


#[derive(Debug, Serialize, Deserialize)]
pub struct SeriesData{
    items_lists:SeriesList,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SeriesList{
    page:CollectionPage,
    series_list:Vec<SeriesItem>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SeriesItem{
    recent_aids:Vec<u64>,
    archives:Vec<CollectArchi>,
    meta:SeriesItemMeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeriesItemMeta{
    cover:String,
    ctime:u64,
    description:String,
    keywords:Vec<String>,
    last_update_ts:u64,
    mid:u64,
    mtime:u64,
    name:String,
    raw_keywords:String,
    series_id:u64,
    total:u64,
}