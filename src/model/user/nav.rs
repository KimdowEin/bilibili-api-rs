use crate::model::sign::wbi::Wbi;
use serde::{Deserialize, Serialize};

///导航栏
#[derive(Debug, Deserialize, Serialize)]
pub struct NavInfo {
    #[serde(rename = "isLogin")]
    pub is_login: bool,
    ///用户信息
    #[serde(flatten, default)]
    pub nav: UserNav,
    /// Wbi 签名实时口令
    pub wbi_img: Wbi,
}


/// 导航栏用户信息
#[derive(Debug, Deserialize, Serialize)]
pub struct UserNav {
    /// 是否验证邮箱
    pub email_verified: bool,
    /// 头像
    pub face: String,
    /// 等级信息
    pub level_info: LevelInfo,
    /// 用户id
    pub mid: u64,
    /// 是否验证手机
    pub mobile_verified: bool,
    /// 硬币数
    pub money: u64,
    /// 节操值
    /// 老古董了，就是信誉值，不确定现在还有没有用
    pub moral: i16,
    /// 认证信息
    pub official: Official,
    /// 认证信息
    #[serde(rename = "officialVerify")]
    pub official2: Official2,

    // pendant: Pendant,

    /// 用户名
    pub uname: String,
    /// vip相关
    #[serde(flatten, default)]
    pub vip: Vip,
    /// 钱包信息
    pub wallet: Wallet,
    /// 有无推广商品
    pub has_shop: bool,
    /// 推广商品链接
    pub shop_url: String,
    /// 是否是风纪委员
    pub is_jury: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LevelInfo {
    /// 当前等级
    pub current_level: u8,
    /// 当前等级经验最低值
    pub current_min: u64,
    /// 当前经验
    pub current_exp: u64,
    /// 下级等级经验
    pub next_exp: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Official {
    /// todo
    pub role: u64,
    pub title: String,
    pub desc: String,
    #[serde(rename = "type")]
    pub is_verified: u8,
}
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Official2 {
    #[serde(rename = "type")]
    pub is_verified: u8,
    pub desc: String,
}

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
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VipLabel {
    pub text: String,
    pub label_theme: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Wallet {
    pub mid: u64,
    pub bcoin_balabce: u64,
    pub coupon_balance: u64,
}
