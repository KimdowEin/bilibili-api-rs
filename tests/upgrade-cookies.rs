use bili_service::SessionState;

#[ignore]
#[tokio::test]
async fn upgrade_cookies() {
    SessionState::from_path("./cookies.json")
        .unwrap()
        .save_cookies("./cookies_v2.json")
        .await
        .unwrap();
}
