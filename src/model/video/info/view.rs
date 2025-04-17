//! 视频信息

use super::{
    cids::Cids,
    desc::{VideoDesc, VideoDesc2},
    state::{Dimension, Rights, VideoCopyRight, VideoStat, VideoState,UpowerState},
    subtitle::Subtitle, 
    zone::Zone
};
use crate::model::user::account::{OwnerCard, Staff, UserInfoBase};
use crate::Data;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::{
    deserialize_bool_from_anything, deserialize_datetime_utc_from_seconds,
    deserialize_default_from_empty_object,
};

/// 视频信息概览
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct VideoView {
    /// 稿件avid
    pub aid: u64,
    /// 稿件bvid
    pub bvid: String,
    /// 视频1P的cid
    pub cid: u64,
    /// 稿件类型 1:原创 2:转载
    pub copyright: VideoCopyRight,
    /// 投稿时间
    #[serde(deserialize_with = "deserialize_datetime_utc_from_seconds")]
    pub ctime: DateTime<Utc>,
    /// 视频简介
    pub desc: VideoDesc,
    /// 新版视频简介
    #[serde(deserialize_with = "deserialize_default_from_empty_object")]
    pub desc_v2: Vec<VideoDesc2>,
    /// 视频1P的分辨率
    pub dimension: Dimension,
    /// 未知
    pub disable_show_up_info: bool,
    /// 稿件总时长(所有分P)
    pub duration: u64,
    /// 视频同步发布的的动态的文字内容
    pub dynamic: String,
    /// 撞车视频跳转avid
    pub forward: Option<u64>,
    /// 未知
    pub is_chargeable_season: bool,
    /// 未知
    pub is_season_display: bool,
    /// 是否可以在 Story Mode 展示
    pub is_story: bool,
    /// 未知
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_story_play: bool,
    /// 充电信息
    #[serde(flatten)]
    pub upower: UpowerState,
    /// 私密
    pub is_view_self: bool,
    /// 稿件参加的活动id
    pub mission_id: Option<u64>,
    /// 未知
    pub need_jump_bv: bool,
    /// UP主信息
    pub owner: UserInfoBase,
    /// 视频分P列表
    pub pages: Vec<Cids>,
    /// 封面图片url
    pub pic: String,
    /// 发布时间
    #[serde(deserialize_with = "deserialize_datetime_utc_from_seconds")]
    pub pubdate: DateTime<Utc>,
    /// 视频属性标志
    pub rights: Rights,
    /// 合作成员列表
    /// todo
    #[serde(default)]
    pub staff: Vec<Staff>,
    /// 视频统计数据
    pub stat: VideoStat,
    /// 稿件状态
    pub state: VideoState,
    /// 视频CC字幕信息
    pub subtitle: Option<Subtitle>,
    /// 青少年模式(未知)
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub teenage_mode: bool,
    /// 分区tid,将要下线
    pub tid: Zone,
    /// 分区tid,文档还未解析
    pub tid_v2: i64,
    /// 稿件标题
    pub title: String,
    /// 子分区名称
    pub tname: String,
    /// 子分区名称
    pub tname_v2: String,
    // user_garb todo
    /// 稿件分P总数
    pub videos: u64,
}


/// 视频页详细信息 别用这个
///
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoInfo {
    /// 视频基本信息
    #[serde(rename = "View")]
    pub view: VideoView,
    #[serde(rename = "Card")]
    /// 视频up主信息
    pub card: OwnerCard,
    // /// 视频Tag信息
    // #[serde(rename = "Tags")]
    // pub tags: Tags,
    // /// 视频热评信息
    // #[serde(rename = "Reply")]
    // pub reply: Reply,
    // /// 视频相关推荐
    // #[serde(rename = "Related")]
    // pub related: Vec<Related>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reply {}
#[derive(Debug, Serialize, Deserialize)]
pub struct Related {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_video_view() {
        let json = r#"{
            "bvid": "BV1VxdyYFEFX",
            "aid": 114314849552940,
            "videos": 1,
            "tid": 25,
            "tid_v2": 2020,
            "tname": "MMD·3D",
            "tname_v2": "翻唱",
            "copyright": 1,
            "pic": "http://i0.hdslb.com/bfs/archive/00cfd50e7ee3d770ed26ad78ea45778b31950d99.jpg",
            "title": "HIMEHINA『心灵感应』Cover",
            "pubdate": 1744373400,
            "ctime": 1744306266,
            "desc": "🌸为纪念频道100万关注，首款手办现正开放预约中✨\n请将小小的HIMEHINA带回家吧！\n\n寿屋官方商城：https://shop.kotobukiya.co.jp/shop/g/g4934054062720/\namiami：https://www.amiami.jp/top/detail/detail?scode=FIGURE-184126\n\n🫧新专辑『Bubblin』开放预约中！\n收录『爱派Dancehall』、『Kiss Kitsune』等多首人气歌曲！\n特设网页 / https://himehina.jp/pages/4th-album-bubblin\n\n🎉11/22-23 将在Pacifico Yokohama举办『Lifetime is Bubblin』2Days演唱会！\n\nPlaylist--------------------------------------------------------\n原创曲 / https://space.bilibili.com/296909317/channel/seriesdetail?sid=2457707\n翻唱曲 / https://space.bilibili.com/296909317/channel/collectiondetail?sid=3017823\n奥术魔刃 / https://space.bilibili.com/296909317/channel/seriesdetail?sid=2457734\n------------------------------------------------------------------\n\n本家：DECO*27\nhttps://youtu.be/c56TpxfO9q0?si=Ee9YLq8to5T1F9ml\n© DECO*27 / © OTOIRO\n\n歌：HIMEHINA(田中姬＋铃木雏)\n  http://twitter.com/HimeTanaka_HH / http://twitter.com/HinaSuzuki_HH\n\nCreative Dir.：夏虫 https://twitter.com/natsu6si\nSupervisor.：Yurio https://twitter.com/yurio0000\nIllust：柳田椎渚\nMovie：はでがみちゃん\nSound Dir.：大山徹也\nRec：すずきゆうか\nMix：飯波光洋\n\n＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-\nHIME TANAKA http://twitter.com/HimeTanaka_HH\nHINA SUZUKI   http://twitter.com/HinaSuzuki_HH\nTikTok         https://www.tiktok.com/@himehina.80\nInstagram      https://www.instagram.com/himehina_official/\nWeb           https://www.lara.inc/\nFanclub        https://himehina.jp/\nDiscord        https://discord.gg/XJbKHW2g9t\nStudio LaRa    https://twitter.com/LaRa_km10\n＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-\n\nAll Produced by Studio LaRa",
            "desc_v2": [
            {
                "raw_text": "🌸为纪念频道100万关注，首款手办现正开放预约中✨\n请将小小的HIMEHINA带回家吧！\n\n寿屋官方商城：https://shop.kotobukiya.co.jp/shop/g/g4934054062720/\namiami：https://www.amiami.jp/top/detail/detail?scode=FIGURE-184126\n\n🫧新专辑『Bubblin』开放预约中！\n收录『爱派Dancehall』、『Kiss Kitsune』等多首人气歌曲！\n特设网页 / https://himehina.jp/pages/4th-album-bubblin\n\n🎉11/22-23 将在Pacifico Yokohama举办『Lifetime is Bubblin』2Days演唱会！\n\nPlaylist--------------------------------------------------------\n原创曲 / https://space.bilibili.com/296909317/channel/seriesdetail?sid=2457707\n翻唱曲 / https://space.bilibili.com/296909317/channel/collectiondetail?sid=3017823\n奥术魔刃 / https://space.bilibili.com/296909317/channel/seriesdetail?sid=2457734\n------------------------------------------------------------------\n\n本家：DECO*27\nhttps://youtu.be/c56TpxfO9q0?si=Ee9YLq8to5T1F9ml\n© DECO*27 / © OTOIRO\n\n歌：HIMEHINA(田中姬＋铃木雏)\n  http://twitter.com/HimeTanaka_HH / http://twitter.com/HinaSuzuki_HH\n\nCreative Dir.：夏虫 https://twitter.com/natsu6si\nSupervisor.：Yurio https://twitter.com/yurio0000\nIllust：柳田椎渚\nMovie：はでがみちゃん\nSound Dir.：大山徹也\nRec：すずきゆうか\nMix：飯波光洋\n\n＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-\nHIME TANAKA http://twitter.com/HimeTanaka_HH\nHINA SUZUKI   http://twitter.com/HinaSuzuki_HH\nTikTok         https://www.tiktok.com/@himehina.80\nInstagram      https://www.instagram.com/himehina_official/\nWeb           https://www.lara.inc/\nFanclub        https://himehina.jp/\nDiscord        https://discord.gg/XJbKHW2g9t\nStudio LaRa    https://twitter.com/LaRa_km10\n＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-\n\nAll Produced by Studio LaRa",
                "type": 1,
                "biz_id": 0
            }
            ],
            "state": 0,
            "duration": 145,
            "rights": {
            "bp": 0,
            "elec": 0,
            "download": 1,
            "movie": 0,
            "pay": 0,
            "hd5": 1,
            "no_reprint": 1,
            "autoplay": 1,
            "ugc_pay": 0,
            "is_cooperation": 0,
            "ugc_pay_preview": 0,
            "no_background": 0,
            "clean_mode": 0,
            "is_stein_gate": 0,
            "is_360": 0,
            "no_share": 0,
            "arc_pay": 0,
            "free_watch": 0
            },
            "owner": {
            "mid": 296909317,
            "name": "田中姬铃木雏Official",
            "face": "https://i1.hdslb.com/bfs/face/49f8c7c45bab6beb503f5bf4fab76fd9bd963f32.jpg"
            },
            "stat": {
            "aid": 114314849552940,
            "view": 79664,
            "danmaku": 86,
            "reply": 198,
            "favorite": 5718,
            "coin": 3506,
            "share": 733,
            "now_rank": 0,
            "his_rank": 0,
            "like": 15521,
            "dislike": 0,
            "evaluation": "",
            "vt": 0
            },
            "argue_info": {
            "argue_msg": "",
            "argue_type": 0,
            "argue_link": ""
            },
            "dynamic": "姬：💛🌀『心灵感应』Cover🌀💛\n\n心灵感应投稿了哦～！！ᐡ⸝⸝\u003E ·̫ \u003C⸝⸝ᐡ💗\n第二段的超有趣舞蹈，我也想跳！这么说了之后拉拉面兴高采烈地帮我录了下来哦🤣！\n还有超怀念的关注画面和meme等，看点满满的MV🥰❣️\n要无限循环哦🫶\n\n雏：⚡️📡『心灵感应』公开！📡⚡️\n\nDECO*27的『心灵感应』投稿了哦！😘💗\n光是唱着就很开心～❣️🥰✨\n\nMV也有做一点改编跳一点舞蹈，还有合体⁉️😳满满有趣的细节，要仔细欣赏哦🥳🌟",
            "cid": 29344663296,
            "dimension": {
            "width": 3840,
            "height": 2160,
            "rotate": 0
            },
            "season_id": 3017823,
            "premiere": null,
            "teenage_mode": 0,
            "is_chargeable_season": false,
            "is_story": false,
            "is_upower_exclusive": false,
            "is_upower_play": false,
            "is_upower_preview": false,
            "enable_vt": 0,
            "vt_display": "",
            "is_upower_exclusive_with_qa": false,
            "no_cache": false,
            "pages": [
            {
                "cid": 29344663296,
                "page": 1,
                "from": "vupload",
                "part": "HIMEHINA『心灵感应』Cover",
                "duration": 145,
                "vid": "",
                "weblink": "",
                "dimension": {
                "width": 3840,
                "height": 2160,
                "rotate": 0
                },
                "first_frame": "http://i1.hdslb.com/bfs/storyff/n250411ad1ayk1pif4m2r61z1c278e6r_firsti.jpg",
                "ctime": 1744306266
            }
            ],
            "subtitle": {
            "allow_submit": false,
            "list": []
            },
            "is_season_display": true,
            "user_garb": {
            "url_image_ani_cut": "https://i0.hdslb.com/bfs/garb/item/e5b841323a75fb454552acf9903a4404eb625011.bin"
            },
            "honor_reply": {

            },
            "like_icon": "",
            "need_jump_bv": false,
            "disable_show_up_info": false,
            "is_story_play": 1,
            "is_view_self": false
        }"#;
        serde_json::from_str::<VideoView>(json).unwrap();
    }
}
