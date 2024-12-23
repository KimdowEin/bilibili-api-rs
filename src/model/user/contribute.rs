use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ContributeView {
    pub archive: Archive,
    pub article: Article,
    pub like: u32,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Archive {
    pub view: u32,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Article {
    pub view: u32,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct AlbumContributeView {
    pub all_count: u32,
    pub draw_count: u32,
    pub photo_count: u32,
    pub daily_count: u32,
}
