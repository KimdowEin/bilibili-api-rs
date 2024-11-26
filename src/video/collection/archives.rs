use serde::{Deserialize, Serialize};

use crate::common::{Query, WbiSign};

use super::{CollectArchi, CollectionPage};



pub const SEASONS_ARCHIVES_LIST_URL: &str =
    "https://api.bilibili.com/x/polymer/web-space/seasons_archives_list";

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchivesListQuery {
    mid: u64,
    season_id: u64,
    sort: bool,
    page_num: u64,
    page_size: u64,
}
impl Query for ArchivesListQuery {}
impl WbiSign for ArchivesListQuery {}


#[derive(Debug, Serialize, Deserialize)]
pub struct ArchivesListData{
    aids: Vec<u64>,
    archives: Vec<CollectArchi>,
    meta: ArchivesListMeta,
    page: CollectionPage,
}




#[derive(Debug, Serialize, Deserialize)]
pub struct ArchivesListMeta {
    cover:String,
    description: String,
    mid: u64,
    name: String,
    ptime: u64,
    season_id: u64,
    total:u64,
}

