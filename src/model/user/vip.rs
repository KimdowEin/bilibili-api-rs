use serde::{Deserialize, Serialize};


#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Vip {
    /// 有无大会员
    #[serde(rename = "vipStatus")]
    pub vip_status: u8,
    /// 大会员到期时间
    #[serde(rename = "VipDueDate")]
    pub vip_due_date: i32,
    /// 大会员类型
    #[serde(rename = "vipType")]
    pub vip_type: u8,
    /// 有无大会员
    pub vip_pay_type: u8,
    /// 怀疑是愚人节彩蛋
    pub vip_theme_type: u64,
    /// 会员标签
    pub vip_label: VipLabel,
    ///是否显示会员图标
    pub vip_avatar_subscript: u8,
    /// 会员昵称颜色
    pub vip_nickname_color: String,
    /// 是否硬核会员
    pub is_senior_member: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vip2 {
    #[serde(rename = "type")]
    pub vip_type: u8,
    pub status: u8,
    pub due_data: Option<u64>,

    pub vip_pay_type: u8,
    pub label: VipLabel2,
    pub avatar_subscript: u8,
    pub nickname_color: String,
    pub role: u8,
    pub avatar_subscript_url: String,
    pub tv_vip_status: u8,
    pub tv_vip_pay_type: u8,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VipLabel {
    pub text: String,
    pub label_theme: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct VipLabel2 {
    #[serde(flatten)]
    pub vip_label:VipLabel,

    pub text_color: String,
    pub bg_color: String,
    pub border_color: String,
    pub img_label_uri_hans_static: String,
    pub img_label_uri_hant_static: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VipStat{
    pub mid: u64,
    pub vip_type: u8,
    pub vip_status: u8,
    pub vip_due_date: u64,
    pub vip_pay_type: bool,
    pub theme_type: u8,
}
