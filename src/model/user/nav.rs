//! 导航栏用户信息
//! 
//! 这个唯一的用处就是获得wbi
//! 
//! 如果想获得用户信息,应该用user/account里的接口

use super::{
    exp::LevelInfo, money::Wallet, official::{Official, OfficialVerify}, pendant::Pendant, vip::Vip
};
use crate::model::sign::wbi::Wbi;
use bili_core::Data;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_bool_from_anything;

///导航栏   
/// 
/// https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/login/login_info.md#%E5%AF%BC%E8%88%AA%E6%A0%8F%E7%94%A8%E6%88%B7%E4%BF%A1%E6%81%AF
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Data)]
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
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UserNav {
    /// 是否验证邮箱
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub email_verified: bool,
    /// 头像
    pub face: String,
    /// 等级信息
    pub level_info: LevelInfo,
    /// 用户id
    pub mid: u64,
    /// 是否验证手机
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub mobile_verified: bool,
    /// 硬币数
    pub money: f64,
    /// 节操值
    /// 老古董了，就是信誉值，不确定现在还有没有用
    pub moral: i16,
    /// 认证信息
    pub official: Official,
    /// 认证信息
    #[serde(rename = "officialVerify")]
    pub official_verify: OfficialVerify,
    /// 头像框
    pub pendant: Pendant,
    /// 用户名
    pub uname: String,
    /// vip相关
    pub vip: Vip,
    /// 钱包信息
    pub wallet: Wallet,
    /// 有无推广商品
    pub has_shop: bool,
    /// 推广商品链接
    pub shop_url: String,

    // pub answer_status: u8,
    
    /// 是否硬核会员
    pub is_senior_member: u8,
    /// 是否是风纪委员
    pub is_jury: bool,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_nav_info() {
        let json_str = r#"
            {
                "isLogin": true,
                "email_verified": 0,
                "face": "https://i2.hdslb.com/bfs/face/348c9b8e1d051b78e56e323c2f44063b79b95c73.jpg",
                "face_nft": 0,
                "face_nft_type": 0,
                "level_info": {
                "current_level": 5,
                "current_min": 10800,
                "current_exp": 24805,
                "next_exp": 28800
                },
                "mid": 200435669,
                "mobile_verified": 1,
                "money": 460.1,
                "moral": 70,
                "official": {
                "role": 0,
                "title": "",
                "desc": "",
                "type": -1
                },
                "officialVerify": {
                "type": -1,
                "desc": ""
                },
                "pendant": {
                "pid": 6335,
                "name": "HINA",
                "image": "https://i2.hdslb.com/bfs/garb/item/5d61ca63a09dbc6617ee980db7e45afec4ca4a6b.png",
                "expire": 0,
                "image_enhance": "https://i2.hdslb.com/bfs/garb/item/5d61ca63a09dbc6617ee980db7e45afec4ca4a6b.png",
                "image_enhance_frame": "",
                "n_pid": 6335
                },
                "scores": 0,
                "uname": "uname",
                "vipDueDate": 1626451200000,
                "vipStatus": 0,
                "vipType": 1,
                "vip_pay_type": 0,
                "vip_theme_type": 0,
                "vip_label": {
                "path": "",
                "text": "",
                "label_theme": "",
                "text_color": "",
                "bg_style": 0,
                "bg_color": "",
                "border_color": "",
                "use_img_label": true,
                "img_label_uri_hans": "",
                "img_label_uri_hant": "",
                "img_label_uri_hans_static": "https://i0.hdslb.com/bfs/vip/d7b702ef65a976b20ed854cbd04cb9e27341bb79.png",
                "img_label_uri_hant_static": "https://i0.hdslb.com/bfs/activity-plat/static/20220614/e369244d0b14644f5e1a06431e22a4d5/KJunwh19T5.png"
                },
                "vip_avatar_subscript": 0,
                "vip_nickname_color": "",
                "vip": {
                "type": 1,
                "status": 0,
                "due_date": 1626451200000,
                "vip_pay_type": 0,
                "theme_type": 0,
                "label": {
                    "path": "",
                    "text": "",
                    "label_theme": "",
                    "text_color": "",
                    "bg_style": 0,
                    "bg_color": "",
                    "border_color": "",
                    "use_img_label": true,
                    "img_label_uri_hans": "",
                    "img_label_uri_hant": "",
                    "img_label_uri_hans_static": "https://i0.hdslb.com/bfs/vip/d7b702ef65a976b20ed854cbd04cb9e27341bb79.png",
                    "img_label_uri_hant_static": "https://i0.hdslb.com/bfs/activity-plat/static/20220614/e369244d0b14644f5e1a06431e22a4d5/KJunwh19T5.png"
                },
                "avatar_subscript": 0,
                "nickname_color": "",
                "role": 0,
                "avatar_subscript_url": "",
                "tv_vip_status": 0,
                "tv_vip_pay_type": 0,
                "tv_due_date": 0,
                "avatar_icon": {
                    "icon_resource": {

                    }
                }
                },
                "wallet": {
                "mid": 200435669,
                "bcoin_balance": 0,
                "coupon_balance": 0,
                "coupon_due_time": 0
                },
                "has_shop": false,
                "shop_url": "",
                "answer_status": 0,
                "is_senior_member": 0,
                "wbi_img": {
                "img_url": "https://i0.hdslb.com/bfs/wbi/a.png",
                "sub_url": "https://i0.hdslb.com/bfs/wbi/b.png"
                },
                "is_jury": false,
                "name_render": null
            }
        "#;
        serde_json::from_str::<NavInfo>(json_str).unwrap();
    }
}
