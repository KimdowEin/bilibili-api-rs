use serde::{Deserialize, Serialize};

use crate::model::{user::account::{Owner, OwnerCard, Staff}, video::zone::Zone};

use super::{cids::Cids, desc::{VideoDesc, VideoDesc2}, state::{Dimension, Rights, VideoStat, VideoState}};

/// 视频信息概览
/// https://gitee.com/KimdowEin/bilibili-API-collect/blob/master/docs/video/info.md#%E8%8E%B7%E5%8F%96%E8%A7%86%E9%A2%91%E8%AF%A6%E7%BB%86%E4%BF%A1%E6%81%AFweb%E7%AB%AF
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoView {
    /// 稿件avid
    pub aid: u64,
    /// 稿件bvid
    pub bvid: String,
    /// 投稿时间
    pub ctime: u64,
    /// 稿件总时长(所有分P)
    pub duration: u64,
    /// 封面图片url
    pub pic: String,
    /// 稿件分P总数
    pub videos: u64,
    /// 分区tid
    pub tid: Zone,
    /// 子分区名称
    pub tname: String,
    /// 稿件类型 1:原创 2:转载
    pub copyright: u8,
    /// 稿件标题
    pub title: String,
    /// 发布时间
    pub pubdate: u64,
    /// 视频简介
    pub desc: VideoDesc,
    /// 新版视频简介
    pub desc_v2: Vec<VideoDesc2>,
    /// 稿件状态
    pub state: VideoState,
    /// 撞车视频跳转avid
    pub forward: Option<u64>,
    /// 稿件参加的活动id
    pub mission_id: Option<u64>,
    /// 重定向url
    pub redirect_url: Option<String>,
    /// 视频属性标志
    pub rights: Rights,
    /// UP主信息
    pub owner: Owner,
    /// 视频状态数
    pub stat: VideoStat,
    /// 视频同步发布的的动态的文字内容
    pub dynamic: String,
    /// 视频1P的cid
    pub cid: u64,
    /// 视频1P的分辨率
    pub dimension: Dimension,
    /// 视频分P列表
    pub pages: Vec<Cids>,
    /// 视频CC字幕信息
    pub subtitle: Option<Subtitle>,
    /// 合作成员列表
    #[serde(default)]
    pub staff: Vec<Staff>,
}



#[derive(Debug, Default, Serialize, Deserialize, )]
pub struct Subtitle {
    ///是否允许提交字幕
    pub allow_submit: bool,
    /// 字幕列表
    pub list: Vec<SubtitleItem>,
}

#[derive(Debug, Default, Serialize, Deserialize, )]
pub struct SubtitleItem {
    /// 字幕id
    pub id: u64,
    /// 字幕语言
    pub lan: String,
    /// 字幕语言名称
    pub lan_doc: String,
    /// 是否锁定
    pub is_lock: bool,
    /// 作者mid
    pub author_mid: Option<u64>,
    /// json格式字幕文件url
    pub subtitle_url: String,

    // author
}

/// 视频页详细信息   
/// https://gitee.com/KimdowEin/bilibili-API-collect/blob/master/docs/video/info.md#%E8%8E%B7%E5%8F%96%E8%A7%86%E9%A2%91%E8%B6%85%E8%AF%A6%E7%BB%86%E4%BF%A1%E6%81%AFweb%E7%AB%AF
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
        let json = r#"
        {
            "bvid": "BV1QVtfejExd",
            "aid": 113168680551875,
            "videos": 1,
            "tid": 31,
            "tid_v2": 2020,
            "tname": "翻唱",
            "tname_v2": "翻唱",
            "copyright": 1,
            "pic": "http://i0.hdslb.com/bfs/archive/3eab8355deeb4201fbb0ed4e3b3feb46a73d4c55.jpg",
            "title": "HIMEHINA『Never Fiction』Cover",
            "pubdate": 1726834200,
            "ctime": 1726817375,
            "desc": "㊗MV『爱派Dancehall』油管3000万播放🎉纪念周边开始预约！\n『爱派Dancehall亚克力立牌』登场！\n详情 / https://t.co/Uhjro5ta81\n\n🗾HIMEHINA 全国巡回2024(西)『眼泪的香味』门票抽选开放中！\n将在福冈・大阪・名古屋举办！还有FC超先行限定全通票！\n详情 / https://bit.ly/3LTr6GD\n\n💠HIMEHINA LIVE 2024 『眼泪的香味』Blu-ray发售决定＆预约开放中！\n初回限定豪华盘内有满满的特典！\n详情 / https://bit.ly/3yudePQ\n\n🎧 HIMEHINA × ONKYO 联名耳机开始预约！🎊\n搭载了HIMEHINA语音的耳机在8月30日(五)14点预售START！\n预约 / https://onkyodirect.jp/shop/pages/hmhn.aspx\n\n🎤 HIMEHINA×DAM×卡啦OK BanBan联动活动进行中！\nHIMEHINA联动包厢、联动饮料以及能够得到原创奖品的『唱歌活动』！\n详情 /https://himehina.jp/contents/772629\n\nPlaylist--------------------------------------------------------\n原创曲 / https://space.bilibili.com/296909317/channel/seriesdetail?sid=2457707\n翻唱曲 / https://space.bilibili.com/296909317/channel/collectiondetail?sid=3017823\n奥术魔刃 / https://space.bilibili.com/296909317/channel/seriesdetail?sid=2457734\n------------------------------------------------------------------\n\n本家：Kanaria×星街彗星\nhttps://www.youtube.com/watch?v=BuDK3aiMmIs\n\n歌：HIMEHINA(田中姬＋铃木雏)\n　    http://twitter.com/HimeTanaka_HH / http://twitter.com/HinaSuzuki_HH\n\nCreative Dir.：夏虫 https://twitter.com/natsu6si\nSupervisor.：Yurio\nIllust：柳田椎渚\nMovie：ぬっこ\nSound Dir.：大山徹也\nRec：すずきゆうか\nMix：飯波光洋\n\n＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-\n田中姬 Twitter：http://twitter.com/HimeTanaka_HH\n铃木雏 Twitter：http://twitter.com/HinaSuzuki_HH\nStudio LaRa Twitter：https://twitter.com/LaRa_km10\nTikTok：https://www.tiktok.com/@himehina.80\nFC：https://himehina.jp/\nStudio LaRa公式HP：https://www.lara.inc\n＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-\n\nAll Produced by Studio LaRa",
            "desc_v2": [
                {
                    "raw_text": "㊗MV『爱派Dancehall』油管3000万播放🎉纪念周边开始预约！\n『爱派Dancehall亚克力立牌』登场！\n详情 / https://t.co/Uhjro5ta81\n\n🗾HIMEHINA 全国巡回2024(西)『眼泪的香味』门票抽选开放中！\n将在福冈・大阪・名古屋举办！还有FC超先行限定全通票！\n详情 / https://bit.ly/3LTr6GD\n\n💠HIMEHINA LIVE 2024 『眼泪的香味』Blu-ray发售决定＆预约开放中！\n初回限定豪华盘内有满满的特典！\n详情 / https://bit.ly/3yudePQ\n\n🎧 HIMEHINA × ONKYO 联名耳机开始预约！🎊\n搭载了HIMEHINA语音的耳机在8月30日(五)14点预售START！\n预约 / https://onkyodirect.jp/shop/pages/hmhn.aspx\n\n🎤 HIMEHINA×DAM×卡啦OK BanBan联动活动进行中！\nHIMEHINA联动包厢、联动饮料以及能够得到原创奖品的『唱歌活动』！\n详情 /https://himehina.jp/contents/772629\n\nPlaylist--------------------------------------------------------\n原创曲 / https://space.bilibili.com/296909317/channel/seriesdetail?sid=2457707\n翻唱曲 / https://space.bilibili.com/296909317/channel/collectiondetail?sid=3017823\n奥术魔刃 / https://space.bilibili.com/296909317/channel/seriesdetail?sid=2457734\n------------------------------------------------------------------\n\n本家：Kanaria×星街彗星\nhttps://www.youtube.com/watch?v=BuDK3aiMmIs\n\n歌：HIMEHINA(田中姬＋铃木雏)\n　    http://twitter.com/HimeTanaka_HH / http://twitter.com/HinaSuzuki_HH\n\nCreative Dir.：夏虫 https://twitter.com/natsu6si\nSupervisor.：Yurio\nIllust：柳田椎渚\nMovie：ぬっこ\nSound Dir.：大山徹也\nRec：すずきゆうか\nMix：飯波光洋\n\n＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-\n田中姬 Twitter：http://twitter.com/HimeTanaka_HH\n铃木雏 Twitter：http://twitter.com/HinaSuzuki_HH\nStudio LaRa Twitter：https://twitter.com/LaRa_km10\nTikTok：https://www.tiktok.com/@himehina.80\nFC：https://himehina.jp/\nStudio LaRa公式HP：https://www.lara.inc\n＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-＋-\n\nAll Produced by Studio LaRa",
                    "type": 1,
                    "biz_id": 0
                }
            ],
            "state": 0,
            "duration": 133,
            "rights": {
                "bp": 0,
                "elec": 0,
                "download": 1,
                "movie": 0,
                "pay": 0,
                "hd5": 0,
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
                "aid": 113168680551875,
                "view": 71418,
                "danmaku": 57,
                "reply": 154,
                "favorite": 3800,
                "coin": 2991,
                "share": 477,
                "now_rank": 0,
                "his_rank": 0,
                "like": 6133,
                "dislike": 0,
                "evaluation": "",
                "vt": 0
            },




            "argue_info": {
                "argue_msg": "",
                "argue_type": 0,
                "argue_link": ""
            },
            "dynamic": "姬：🪷HIMEHINA『Never Fiction』Cover🪷\n\n翻唱投稿了哦❣️\n听到星街彗星和Kanaria的演唱太喜欢就翻唱了！🥰\n这次努力融入了气泡音和假声等变化，希望大家能听听看🤭💗\n\n雏：❤🥀『Never Fiction』公开！🥀💙\n\n无论是歌声还是插画都充满了成熟的氛围～🤭❣有没有展现出了我们的另一面呢⁉️🤭💓\n仔细听听两人和谐的和声的契合吧😘🫧\n\n希望大家能带上标签发感想到X💌💝",
            "cid": 25931745323,
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
            "no_cache": false,
            "pages": [
                {
                    "cid": 25931745323,
                    "page": 1,
                    "from": "vupload",
                    "part": "HIMEHINA『Never Fiction』Cover",
                    "duration": 133,
                    "vid": "",
                    "weblink": "",
                    "dimension": {
                        "width": 3840,
                        "height": 2160,
                        "rotate": 0
                    },
                    "first_frame": "http://i2.hdslb.com/bfs/storyff/n240920ad1lisxgghj3tly3sn5h0d2w4_firsti.jpg"
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
            "honor_reply": {},
            "like_icon": "",
            "need_jump_bv": false,
            "disable_show_up_info": false,
            "is_story_play": 1,
            "is_view_self": false
        }
        "#;
        serde_json::from_str::<VideoView>(json).unwrap();

    }
}