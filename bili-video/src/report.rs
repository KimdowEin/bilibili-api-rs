//! 视频观看数据上报

use bili_core::ToQuery;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// 上报观看进度（双端）
pub const HISTORY_REPORT_URL: &str = "https://api.bilibili.com/x/v2/history/report";

/// 上报观看进度（双端）
///
/// csrf(post)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, TypedBuilder)]
pub struct HistoryReportQuery {
    pub aid: u64,
    pub cid: u64,
    #[builder(default, setter(into, strip_option))]
    pub progress: Option<u64>,
    #[builder(default, setter(into, strip_option))]
    pub platform: Option<String>,
}

#[cfg(test)]
mod tests {
    use bili_core::ToQuery;

    use tokio::fs;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_query_history_report() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");
        session.refresh_csrf().unwrap();

        let url = HistoryReportQuery::builder()
            .aid(13662970)
            .cid(126654047)
            .progress(1248u64)
            .platform("android".to_string())
            .build()
            .to_query()
            .unwrap()
            .with_csrf(&session.bili_jct())
            .unwrap()
            .to_url(HISTORY_REPORT_URL);

        let json = session
            .post(url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        fs::write("../tests/datas/video/history_report.json", &json)
            .await
            .unwrap();
    }
}
