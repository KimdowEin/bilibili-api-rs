//! 播放器信息

use bili_core::{Data, ToQuery};
use serde::{Deserialize, Serialize};

use crate::VideoQuery;

/// 获取web播放器信息
pub const PLAYER_INFO_URL: &str = "https://api.bilibili.com/x/player/wbi/v2";

/// 获取web播放器信息
///
/// sign
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct PlayerInfoQuery {
    #[serde(flatten)]
    pub vid: VideoQuery,
    pub cid: u64,
}
impl PlayerInfoQuery {
    pub fn new(vid: VideoQuery, cid: u64) -> Self {
        Self { vid, cid }
    }
}

/// web播放器信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct PlayerInfo {
    pub aid: u64,
    pub bvid: String,
    pub allow_bp: bool,
    pub no_share: bool,
    pub cid: u64,
    /// 智能防挡字幕信息
    pub dm_mask: Option<DmMask>,
    /// 字幕信息
    pub subtitle: PlayerSubtitle,
    /// 分段章节信息
    pub view_points: Vec<ViewPoint>,
    /// 请求IP信息
    pub ip_info: IpInfo,
    /// 登录用户mid
    pub login_mid: u64,
    /// 登录用户mid hash
    pub login_mid_hash: String,
    /// 是否为该视频UP主
    pub is_owner: bool,
    /// 登录用户等级信息
    pub level_info: LevelInfo,
    /// 登录用户VIP信息
    pub vip: VipInfo,
    /// 上次播放时间
    pub last_play_time: u64,
    /// 上次播放cid
    pub last_play_cid: u64,
    /// 当前UNIX秒级时间戳
    pub now_time: u64,
    /// 在线人数
    pub online_count: u64,
    /// 是否必须登录才能查看字幕
    pub need_login_subtitle: bool,
    /// 互动视频资讯
    pub interaction: Option<Interaction>,
    /// 背景音乐信息
    pub bgm_info: Option<BgmInfo>,
    /// 充电专属视频信息
    pub elec_high_level: Option<ElecHighLevel>,
    /// 是否为充电专属视频
    pub is_upower_exclusive: bool,
    pub is_upower_play: bool,
    pub is_ugc_pay_preview: bool,
    pub disable_show_up_info: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DmMask {
    pub cid: u64,
    pub plat: u64,
    pub fps: u64,
    pub time: u64,
    pub mask_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerSubtitle {
    pub allow_submit: bool,
    pub lan: String,
    pub lan_doc: String,
    pub subtitles: Vec<SubtitleItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubtitleItem {
    pub ai_status: u64,
    pub ai_type: u64,
    pub id: u64,
    pub id_str: String,
    pub is_lock: bool,
    pub lan: String,
    pub lan_doc: String,
    pub subtitle_url: String,
    #[serde(rename = "type")]
    pub subtitle_type: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewPoint {
    pub content: String,
    pub from: u64,
    pub to: u64,
    #[serde(rename = "type")]
    pub point_type: u64,
    pub img_url: String,
    pub logo_url: String,
    pub team_type: String,
    pub team_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpInfo {
    pub ip: String,
    pub zone_ip: String,
    pub zone_id: u64,
    pub country: String,
    pub province: String,
    pub city: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LevelInfo {
    pub current_level: u64,
    pub current_min: u64,
    pub current_exp: u64,
    pub next_exp: i64,
    pub level_up: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VipInfo {
    #[serde(rename = "type")]
    pub vip_type: u64,
    pub status: u64,
    pub due_date: u64,
    pub vip_pay_type: u64,
    pub theme_type: u64,
    pub label: VipLabel,
    pub avatar_subscript: u64,
    pub nickname_color: String,
    pub role: u64,
    pub avatar_subscript_url: String,
    pub tv_vip_status: u64,
    pub tv_vip_pay_type: u64,
    pub tv_due_date: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VipLabel {
    pub path: String,
    pub text: String,
    pub label_theme: String,
    pub text_color: String,
    pub bg_style: u64,
    pub bg_color: String,
    pub border_color: String,
    pub use_img_label: bool,
    pub img_label_uri_hans: String,
    pub img_label_uri_hant: String,
    pub img_label_uri_hans_static: String,
    pub img_label_uri_hant_static: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Interaction {
    pub graph_version: u64,
    pub msg: String,
    pub error_toast: String,
    pub mark: u64,
    pub need_reload: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BgmInfo {
    pub music_id: String,
    pub music_title: String,
    pub jump_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElecHighLevel {
    pub privilege_type: u64,
    pub title: String,
    pub sub_title: String,
    pub show_button: bool,
    pub button_text: String,
    pub jump_url: String,
    pub intro: String,
    pub new: bool,
}

#[cfg(test)]
mod tests {
    use bili_core::{BiliResponse, ToQuery};

    use tokio::fs;

    use super::*;

    const BVID: &str = "BV1SWfwY3ENK";
    const CID: u64 = 29193274957;

    #[tokio::test]
    #[ignore]
    async fn test_query_player_info() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = PlayerInfoQuery::new(BVID.into(), CID)
            .to_query()
            .unwrap()
            .with_sign(&session.mixin_key())
            .unwrap()
            .to_url(PLAYER_INFO_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        fs::write("../tests/datas/video/player_info.json", &json)
            .await
            .unwrap();
    }

    #[test]
    fn test_deserialize_player_info() {
        let json = include_str!("../../../tests/datas/video/player_info.json");
        serde_json::from_str::<BiliResponse<PlayerInfo>>(json).unwrap();
    }
}
