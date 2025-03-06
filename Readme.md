# bilibili-api-rs

## 介绍

对b站api的rust封装(建设中)，基于 [bilibili-API-collect](https://github.com/SocialSisterYi/bilibili-API-collect)

会坚持弄完的

v0.2.0建设中,会重构大部分api,并重构部分模块

后续会逐步完成其他不常用的模块,并补充文档

## 快速开始

### 导入库

```toml
# Cargo.toml
[dependencies.bilibili-api-rs]
git = "https://github.com/KimdowEin/bilibili-api-rs"
brance = "night"
features = ["session","manual"]
```

以下是v0.1.0的示例,还没改,但流程差不多

### 登录(password)

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

### 下载视频

```rust
use bilibili_api_rs::{
    model::video::stream::{Fnval, Qn}, query::video::{info::cids::VideoCidsQuery, stream::VideoStreamQuery}, service::session::Session
};
use futures_util::StreamExt;
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::test]
async fn download_video() {
  let bvid = "BV1Zs4y1p7zg";
  let mut session = Session::new_with_path("./cookies.json").unwrap();
  session.refresh_sign().await.unwrap();

  let query = VideoCidsQuery::new(None, Some(bvid));
  let cids = session.get_video_cids(query).await.unwrap();
  let cid = cids[0].cid;

  let query = VideoStreamQuery::new(None,Some(bvid),cid,Some(Qn::FHD),Some(Fnval::MP4),None,None,);
  let durl = session.get_video_stream(query).await.unwrap().durl.unwrap()[0]
      .url
      .clone();
  let mut file = File::options()
      .write(true)
      .create(true)
      .open("./tests/res/test.mp4")
      .await
      .unwrap();
  let response = session.get(durl).send().await.unwrap();
  let pb = ProgressBar::new(response.content_length().unwrap_or_default());
  pb.set_style(
      ProgressStyle::default_bar()
          .template("[{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
          .progress_chars("#>-"),
  );
  let mut stream = response.bytes_stream();
  let mut downloaded = 0;
  while let Some(item) = stream.next().await {
      let bytes = item.unwrap();
      file.write_all(&bytes).await.unwrap();
      downloaded += bytes.len() as u64;
      pb.set_position(downloaded);
  }

  pb.finish_with_message("下载完成");
}

```

### 一般流程

常用的请求session有对应函数减少模板代码,如果没有,按照如下步骤

1. 找到请求体(???Query),生成请求
2. 和url(???_URL)拼接({}?{},url,query)
3. 发起请求
4. 解释响应体(BiliResponse<???Model>)
5. 获得数据(response.data())

```rust
use crate::{
  model::{response::BiliResponse, video::info::desc::VideoDesc}, 
  query::video::info::desc::{VideoDescQuery,VIDEO_DESC_URL}, service::session::Session, 
  traits::Query
};

const BVID: &str = "BV1wDCwYfE2f";

#[tokio::test]
async fn get_video_desc() {

  let session = Session::new_with_path("./cookies.json").unwrap();

  let query = VideoDescQuery::new(None,Some(BVID));
  // 部分需要鉴权,将to_query()替换sign()
  let url = format!("{}?{}",VIDEO_DESC_URL,query.to_query().unwrap());

  let desc = session.get(url)
      .send()
      .await
      .unwrap()
      .json::<BiliResponse<VideoDesc>>()
      .await
      .unwrap()
      .data()
      .unwrap();
}
```

## 功能 feature

- "session" 提供一个会话 一些常用请求模板代码 cookies的导入和保存
- "manual" 提供一个函数跳转到过人机验证的网站

## 提供的模块

打x是完成并测试  
打o是未测试或部分完成

todo

写文档真的好麻烦

## 一些简单说明

- 项目结构
  - query 请求结构体
  - model 响应体
  - service 一些模板代码和request逻辑
- 命名
  - Query Model Url 前缀有app的是app端的api,没有的是web端或两者共用的api
  - 字段前缀有app的是app专属 Option是可选

## 共同建设

需要大量测试用例

存在一些封装不全,忘记pub,结构不合理的地方,欢迎提交pr或issue(去github提pr)
