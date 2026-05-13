use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FansMedal {
    pub uid: i32,
    pub target_id: i32,
    pub medal_id: i32,
    pub level: i32,
    pub medal_name: String,
    pub medal_color: i32,
    pub intimacy: i32,
    pub next_intimacy: i32,
    pub day_limit: u64,
    pub today_feed: u64,
    pub medal_color_start: i32,
    pub medal_color_end: i32,
    pub medal_color_border: i32,
    pub is_lighted: bool,
    pub light_status: i32,
    pub wearing_status: i32,
    pub score: i32,
}
