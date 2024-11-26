use archives::ArchivesListData;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use series::SeriesData;


mod archives;
mod series;

#[derive(Debug,Serialize, Deserialize)]
pub struct CollectionResponse{
    code:CollectionResponseCode,
    message:String,
    data:CollectionData
}

#[derive(Debug,Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum CollectionResponseCode {
    Success=0,
    RequestRisk=-352,
    RequestError=-400,
}

#[derive(Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum CollectionData {
    ArchivesListData(ArchivesListData),
    SeriesData(SeriesData),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionPage {
    page_num: u64,
    page_size: u64,
    total: u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectArchi {
    aid: u64,
    bvid: String,
    ctime: u64,
    duration: u64,
    interactive_video: bool,
    pic:String,
    pubdate:u64,
    stat: CollectArchiStat,
    title: String,
    ugc_pay:u8,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectArchiStat {
    view: u64,
}