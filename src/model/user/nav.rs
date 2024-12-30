use crate::model::sign::wbi::Wbi;
use serde::{Deserialize, Serialize};

use super::{exp::LevelInfo, official::{Official, OfficialVerify}, vip::Vip};

///导航栏
#[derive(Debug, Deserialize, Serialize)]
pub struct NavInfo {
    #[serde(rename = "isLogin")]
    pub is_login: bool,
    ///用户信息
    #[serde(flatten)]
    pub nav: Option<UserNav>,
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
    pub official_verify: OfficialVerify,

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
pub struct Wallet {
    pub mid: u64,
    pub bcoin_balabce: u64,
    pub coupon_balance: u64,
}
