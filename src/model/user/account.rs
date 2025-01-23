use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{exp::LevelView, official::{Official, OfficialVerify}, vip::Vip};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub mid: u64,
    pub uname: String,
    pub userid:String,
    pub sign: String,
    pub birthday: String,
    pub sex: String,
    pub nick_free:bool,
    pub rank:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveAccountInfo {
    pub uid:u64,
    pub uname:String,
    pub face:String,
    pub official_verify:OfficialVerify,
    pub gender:i8,
}
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum GenderType{
    Secrecy = -1,
    Female = 0, 
    Male = 1,
}

#[derive(Debug, Default, Serialize, Deserialize, )]
pub struct Owner {
    /// UP mid
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub mid: u64,
    /// UP昵称
    pub name: String,
    /// UP头像
    pub face: String,
}



/// 视频用户栏信息
#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerCard {
    pub card: CardView,
    pub space: Sapce,
    pub following: bool,
    pub archive_count: u64,
    pub article_count: u64,
    pub follower: u64,
    pub like_num: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardView {
    #[serde(flatten)]
    pub owner:Owner,
    pub sex: String,
    pub face_nft: u8,
    pub birthday: String,
    pub fans: u64,
    pub attention: u64,
    pub sign: String,
    pub level_info: LevelView,
    pub pendant: Pendant,
    pub nameplate: NamePlate,
    #[serde(rename = "Official")]
    pub official: Official,
    pub official_verify: OfficialVerify,
    pub vip: Vip,
    pub is_senior_member: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pendant {
    pub pid: u64,
    pub name: String,
    pub image: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NamePlate {
    pub nid: u64,
    pub name: String,
    pub image: String,
    pub image_small: String,
    pub level: String,
    pub condition: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sapce {
    pub s_img: String,
    pub l_img: String,
}

#[derive(Debug, Default, Serialize, Deserialize, )]
pub struct Staff {
    #[serde(flatten)]
    pub owner: Owner,
    pub title: String, //名称
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_deserialize_owner_card(){
        let json = r#"
        {
            "card":{
                "mid": "296909317",
                "name": "田中姬铃木雏Official",
                "approve": false,
                "sex": "保密",
                "rank": "10000",
                "face": "https://i1.hdslb.com/bfs/face/49f8c7c45bab6beb503f5bf4fab76fd9bd963f32.jpg",
                "face_nft": 0,
                "face_nft_type": 0,
                "DisplayRank": "0",
                "regtime": 0,
                "spacesta": 0,
                "birthday": "",
                "place": "",
                "description": "",
                "article": 0,
                "attentions": [],
                "fans": 360862,
                "friend": 46,
                "attention": 46,
                "sign": "HIMEHINA：请大家多多关照！商务合作请联系官博：田中姬铃木雏Official",
                "level_info": {
                    "current_level": 6,
                    "current_min": 0,
                    "current_exp": 0,
                    "next_exp": 0
                },
                "pendant": {
                    "pid": 0,
                    "name": "",
                    "image": "",
                    "expire": 0,
                    "image_enhance": "",
                    "image_enhance_frame": "",
                    "n_pid": 0
                },
                "nameplate": {
                    "nid": 8,
                    "name": "知名偶像",
                    "image": "https://i1.hdslb.com/bfs/face/27a952195555e64508310e366b3e38bd4cd143fc.png",
                    "image_small": "https://i1.hdslb.com/bfs/face/0497be49e08357bf05bca56e33a0637a273a7610.png",
                    "level": "稀有勋章",
                    "condition": "所有自制视频总播放数>=100万"
                },
                "Official": {
                    "role": 1,
                    "title": "bilibili 知名UP主",
                    "desc": "日本虚拟UP主",
                    "type": 0
                },
                "official_verify": {
                    "type": 0,
                    "desc": "bilibili 知名UP主"
                },
                "vip": {
                    "type": 2,
                    "status": 1,
                    "due_date": 1769356800000,
                    "vip_pay_type": 1,
                    "theme_type": 0,
                    "label": {
                        "path": "",
                        "text": "年度大会员",
                        "label_theme": "annual_vip",
                        "text_color": "%#FFFFFF",
                        "bg_style": 1,
                        "bg_color": "%#FB7299",
                        "border_color": "",
                        "use_img_label": true,
                        "img_label_uri_hans": "",
                        "img_label_uri_hant": "",
                        "img_label_uri_hans_static": "https://i0.hdslb.com/bfs/vip/8d4f8bfc713826a5412a0a27eaaac4d6b9ede1d9.png",
                        "img_label_uri_hant_static": "https://i0.hdslb.com/bfs/activity-plat/static/20220614/e369244d0b14644f5e1a06431e22a4d5/VEW8fCC0hg.png"
                    },
                    "avatar_subscript": 1,
                    "nickname_color": "%#FB7299",
                    "role": 3,
                    "avatar_subscript_url": "",
                    "tv_vip_status": 0,
                    "tv_vip_pay_type": 0,
                    "tv_due_date": 0,
                    "avatar_icon": {
                        "icon_type": 1,
                        "icon_resource": {}
                    },
                    "vipType": 2,
                    "vipStatus": 1
                },
                "is_senior_member": 0,
                "name_render": null
            },
            "space": {
                "s_img": "http://i0.hdslb.com/bfs/space/4fda7448d3974b7b5e709f6f7459324955daef0c.png",
                "l_img": "http://i0.hdslb.com/bfs/space/4fda7448d3974b7b5e709f6f7459324955daef0c.png"
            },
            "following": true,
            "archive_count": 762,
            "article_count": 0,
            "follower": 360862,
            "like_num": 8669753
        }
        "#;
        serde_json::from_str::<OwnerCard>(json).unwrap();
    }
} 