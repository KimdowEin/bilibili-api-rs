use bili_core::Data;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_bool_from_anything;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Data)]
pub struct Vip {
    #[serde(flatten)]
    pub view: VipView,

    #[serde(flatten)]
    pub tv_vip: TvVip,

    /// 大会员标签
    pub label: VipLabel,

    pub avatar: VipAvatar,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct VipView {
    /// 有无大会员
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub status: bool,
    /// 大会员到期时间
    #[serde(alias = "vipDueDate")]
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub due_date: DateTime<Utc>,
    /// 大会员类型
    pub role: VipRole,
    /// 有无自动续费
    #[serde(rename = "vip_pay_type")]
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub auto_pay: bool,

    /// 会员昵称颜色
    #[serde(alias = "vip_nickname_color")]
    pub nickname_color: String,
    #[serde(alias = "vip_theme_type")]
    /// 怀疑是愚人节彩蛋
    pub theme_type: i32,
}

/// 大会员类型
#[derive(Debug, Clone, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum VipRole {
    Normal = 0,
    /// 月度
    Month = 1,
    /// 年度
    Year = 3,
    /// 十年大会员
    TenYear = 7,
    /// 百年大会员
    HundredYear = 15,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VipLabel {
    /// 会员类型文案
    pub text: String,
    /// 会员标签
    pub label_theme: LabelTheme,
    /// 会员标签
    pub text_color: String,
    /// 会员标签背景颜色
    pub bg_color: String,
    /// 会员标签边框颜色
    pub border_color: String,

    pub use_img_label: bool,
    /// 会员牌子图片简体
    pub img_label_uri_hans_static: String,
    /// 会员牌子图片繁体
    pub img_label_uri_hant_static: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LabelTheme {
    Vip,
    AnnualVip,
    TenAnnualVip,
    HundredAnnualVip,
    FoolsDayHundredAnnualVip,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TvVip {
    #[serde(rename = "tv_vip_status")]
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub status: bool,
    #[serde(rename = "tv_vip_pay_type")]
    pub pay_type: i32,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    #[serde(rename = "tv_due_date")]
    pub tv_due_date: DateTime<Utc>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VipAvatar {
    ///是否显示会员图标
    #[serde(alias = "vip_avatar_subscript")]
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub avatar_subscript: bool,
    /// 大会员角标地址
    pub avatar_subscript_url: String,
    pub avatar_icon: VipAvatarIcon,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VipAvatarIcon {
    icon_type: i32,
    // icon_resource: String,
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_deserialize_vip() {
    //     let s = r#"{
    //         "type": 1,
    //         "status": 0,
    //         "due_date": 1493827200000,
    //         "vip_pay_type": 0,
    //         "theme_type": 0,
    //         "label": {
    //             "path": "",
    //             "text": "",
    //             "label_theme": "",
    //             "text_color": "",
    //             "bg_style": 0,
    //             "bg_color": "",
    //             "border_color": "",
    //             "use_img_label": true,
    //             "img_label_uri_hans": "",
    //             "img_label_uri_hant": "",
    //             "img_label_uri_hans_static": "https://i0.hdslb.com/bfs/vip/d7b702ef65a976b20ed854cbd04cb9e27341bb79.png",
    //             "img_label_uri_hant_static": "https://i0.hdslb.com/bfs/activity-plat/static/20220614/e369244d0b14644f5e1a06431e22a4d5/KJunwh19T5.png"
    //         },
    //         "avatar_subscript": 0,
    //         "nickname_color": "",
    //         "role": 0,
    //         "avatar_subscript_url": "",
    //         "tv_vip_status": 0,
    //         "tv_vip_pay_type": 0,
    //         "tv_due_date": 0,
    //         "vipType": 1,
    //         "vipStatus": 0
    //     }"#;

    //     serde_json::from_str::<Vip>(s).unwrap();
    // }
}
