use serde::{Deserialize, Serialize};

use crate::{
    common::{Query, WbiSign},
    video::zone::Zone,
};

pub const VIDEO_CONTRIBUTE_URL: &str = "https://api.bilibili.com/x/space/wbi/arc/search";

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoContributeQuery {
    mid: u64,
    order: Order,
    tid: Zone,
    keyword: String,
    pn: usize,
    ps: u64,
}
impl Query for VideoContributeQuery {}
impl WbiSign for VideoContributeQuery {}

#[derive(Debug, Serialize, Deserialize)]
enum Order {
    Pubdate,
    Click,
    Stow,
}
impl ToString for Order {
    fn to_string(&self) -> String {
        match self {
            Order::Pubdate => "pubdate".to_string(),
            Order::Click => "click".to_string(),
            Order::Stow => "stow".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoContributeData {
    pub list: VideoContributeList,
    pub page: VideoContributePage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoContributeList {
    pub vlist: Vec<VideoContributeItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoContributeItem {
    pub aid: u64,
    pub bvid: String,
    pub author: String,
    pub comment: u64,
    pub created: u64,
    pub description: String,
    pub is_pay: u8,
    pub is_union_video: u8,
    pub length: String,
    pub mid: u64,
    pub pic: String,
    pub play: u64,
    pub title: String,
    // pub typeid: Zone,
    pub video_review: u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoContributePage {
    pub count: u64,
    pub pn: usize,
    pub ps: u64,
}


#[test]
fn test() {
    #[derive(Deserialize, Serialize)]
struct Query {
    name: String,
    age: Option<u8>,
    occupation: String,
}
    let a = serde_qs::to_string(
        &Query{
            name: "John".to_string(),
            age: None,
            occupation: "Engineer".to_string(),
        }
    ).unwrap();
    println!("{}",a)
}