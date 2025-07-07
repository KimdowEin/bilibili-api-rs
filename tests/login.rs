#![cfg(all(feature = "manual", feature = "session"))]

use bilibili_api_rs::{
    query::login::{
        captcha::CaptchaQuery,
        password::{LoginKeyQuery, PasswordLoginQuery},
    },
    service::{
        login::{manual_verification, CaptchaRequest, LoginKeyRequest, PasswordLoginRequest},
        Session,
    },
    traits::BiliRequest,
};

#[tokio::test]
#[ignore = "需要手动在控制台进行输入,测试前将should_panic注释掉"]
#[should_panic]
async fn test_login() {
    let (username, password) = ("username", "password");
    let session = Session::new().unwrap();

    let query = CaptchaQuery::new();
    let captcha = CaptchaRequest::send_request(&session, query).await.unwrap();

    // 这里会使用默认浏览器跳转到一个过captcha的页面，需要手动验证
    manual_verification(&captcha.geetest).unwrap();

    // 将得到的结果 verify 输入到控制台
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let validate = buf.trim();

    let query = LoginKeyQuery::new();
    let key = LoginKeyRequest::send_request(&session, query)
        .await
        .unwrap();
    let password = key.decode_password(password).unwrap();

    let query = PasswordLoginQuery::new(
        username.to_string(),
        password,
        captcha,
        validate.to_string(),
        None,
        None,
    );
    let response = PasswordLoginRequest::send_request(&session, query)
        .await
        .unwrap();

    println!("登录状态: {}", response.message);

    session.save_cookies().unwrap();
}
