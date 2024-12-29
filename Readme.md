### 介绍
对b站api的rust封装(建设中)，基于 [bilibili-API-collect](https://github.com/SocialSisterYi/bilibili-API-collect)

会坚持弄完的

1月下旬增加live模块，提供文档和示例，并提交到rustio

### 快速开始
导入库
```toml
# Cargo.toml
[dependencies]
bilibili-api-rs = {git = "https://github.com/KimdowEin/bilibili-api-rs",features = ["session","manual"]}
```

登录(password)
```rust
// src/main.rs
use bilibili_api_rs::service::session::Session;
use bilibili_api_rs::service::login::manual_verification;

#[tokio::main]
async fn main() {
  let (username, password) = ("username", "password");

  let session = Session::new().unwrap();

  let captcha = session.captcha().await.unwrap();

  // 需要启用 manual
  // 这里会使用默认浏览器跳转到一个过captcha的页面，需要手动验证
  // 将得到的结果 verify 输入到控制台
  manual_verification(&captcha.geetest).unwrap();

  let mut buf = Vec::new();
  tokio::io::stdin().read_buf(&mut buf).await.unwrap();
  let validate= String::from_utf8(buf).unwrap().trim();
  
  let key = session.get_login_key().await.unwrap();
  let password = key.decode_password(password).unwrap();

  let query = LoginQuery::new(username.to_string(), password.to_string(), captcha, validate.to_string(), None, None);
  let response = session.login_by_password(query).await.unwrap();
  println!("登录成功: {:?}", response);

  // 保存 cookies 到文件
  session.save_cookies().unwrap();
}
```


或者直接在浏览器复制cookies(推荐)
```json
// ./cookies.json
[
  {
    "url":"https://api.bilibili.com",
    "cookies":"a=abcdefg; b=hijklmn"
  }
]
```

```rust
// src/main.rs
use bilibili_api_rs::service::session::Session;
#[tokio::main]
async fn main() {
  let session = Session::new_with_path("cookies.json").unwrap();
}
```

下载视频
```rust

```

### 功能 feature
- "session" 提供一个会话 一些常用请求模板代码 cookies的导入和保存 
- "manual" 提供一个函数跳转到过人机验证的网站

### 进度
打x是完成并测试  
打o是完成未测试
- [ ] 接口签名与验证
  - [ ] APP API 签名（appkey与sign）
  - [ ] 已知的 APPKey
  - [x] Wbi 签名（wts与w_rid）
  - [ ] bili_ticket
- [ ] 登录
  - [x] 登录操作 (人机认证)
    - [ ] 短信登录
    - [x] 密码登录
    - [ ] 二维码登录
    - [ ] SNS 登录 (QQ & 微信 & 微博)
  - [x] 登录基本信息
  - [o] 个人中心
  - [ ] 注销登录
  - [ ] 登录记录
  - [ ] Web 端 Cookie 刷新
- [ ] 用户
  - [ ] 基本信息
  - [ ] 状态数
  - [ ] 关系
  - [ ] 个人空间 
  - [ ] 检查昵称是否可注册
  - [ ] 用户注册
  - [ ] 用户认证类型一览
  - [ ] 加入老粉计划
- [ ] 视频
  - [o] 视频分区一览 (分区代码)
  - [o] 基本信息
  - [ ] 快照   
  - [o] 点赞 & 投币 & 收藏 & 分享
  - [ ] TAG   
  - [ ] 视频推荐
  - [o] 播放&下载地址 (视频流)
  - [ ] 互动视频
  - [ ] 高能进度条
  - [ ] 信息上报 (心跳及记录历史)
  - [o] 视频属性数据
  - [ ] 视频在线人数
  - [ ] 视频AI摘要
  - [ ] 稿件投诉
  - [ ] 视频合集
- [ ] 直播
  - [o] 直播间基本信息
  - [ ] 直播分区
  - [ ] 直播间管理
  - [ ] 直播间操作
  - [ ] 直播视频流
  - [ ] 直播信息流
  - [ ] 直播红包
  - [ ] 直播间用户实用 API
  - [ ] 直播间禁言相关
  - [ ] 关注UP直播情况
  - [ ] 直播心跳上报   

初步测试  
可以完成账号密码登录，获取视频信息并完成下载(对大部分人来说够用了)

### 一些简单说明
- 项目结构   
  - query 请求结构体
  - model 响应体
  - service 一些模板代码和session逻辑
- 命名
  - Query Model Url 前缀有app的是app端的api,没有的是web端或两者共用的api
  - 字段前缀有app的是app专属 Option是可选
- 一般步骤
  1. 找到请求体(???Query)
  2. 生成请求
  3. 和url(???_URL)拼接({}?{},url,query)
  4. 发起请求 
  5. 解释响应体(BiliResponse<???Model>)
  6. 获得数据(response.data())


### 共同建设
存在一些封装不全,忘记pub,结构不合理的地方,欢迎提交pr或issue(去github提pr)

