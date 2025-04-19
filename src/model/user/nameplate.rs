use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_bool_from_anything;
/// 勋章信息
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamePlate {
    /// 勋章id
    pub nid: u64,
    /// 勋章名称
    pub name: String,
    /// 勋章图片
    pub image: String,
    /// 勋章图片(小)
    pub image_small: String,
    /// 勋章等级
    pub level: String,
    /// 获取条件
    pub condition: String,
}

/// 粉丝勋章
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

pub struct FansMedal {
    pub show: bool,
    pub wear: bool,
    pub medal: FansMedalInfo,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FansMedalInfo {
    /// 此用户的mid
    pub uid: u64,
    /// 目标用户的id
    pub target_id: u64,
    /// 勋章的id
    pub medal_id: u64,
    /// 勋章的等级
    pub level: u64,
    /// 勋章的名称
    pub medal_name: String,
    /// 勋章的颜色
    pub medal_color: u64,
    /// 当前的亲密度
    pub intimacy: u64,
    /// 下一级亲密度所需的经验值
    pub next_intimacy: u64,
    /// 每日喂食上限
    pub day_limit: u64,
    /// 今日已喂食次数
    pub today_feed: u64,
    /// 勋章颜色的起始值
    pub medal_color_start: u64,
    /// 勋章颜色的结束值
    pub medal_color_end: u64,
    /// 勋章边框颜色
    pub medal_color_border: u64,
    /// 是否点亮
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_lighted: bool,
    /// 点亮状态
    pub light_status: i32,
    /// 是否佩戴
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub wearing_status: bool,
    /// 徽章得分
    pub score: i32,
}
