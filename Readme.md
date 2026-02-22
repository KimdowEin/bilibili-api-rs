# bilibili-api-rs

## 声明

本库仅用于学习和测试,禁止用于任何非法用途,灰产或其他恶劣行为.不当使用产生的所有后果与作者无关.

如果你对此库的存在有任何不满,我立马删库致歉,官方打过来我第一个投降.

![投降](投降.jpg)

## 介绍

对b站api的rust封装(建设中)，基于 [bilibili-API-collect](https://github.com/SocialSisterYi/bilibili-API-collect)

会坚持弄完的

v0.4.0建设中,已重新设计项目结构，进入堆量期

## 快速开始

### 导入库

```toml
# Cargo.toml
[dependencies.bilibili-api-rs]
git = "https://github.com/KimdowEin/bilibili-api-rs"
brance = "night"
```

### 制作cookies

cookies格式有v1、v2两个版本

v1是能够人工写入的，v2则机器易读

v1是兼容的，导入v1后，保存得到的是v2文件

在浏览器复制cookies，按如下格式粘贴

```json
// ./cookies.json
[
  {
    "url":"https://api.bilibili.com",
    "cookies":"a=abcdefg; b=hijklmn"
  }，
  // 将上面的cookies多复制几个，然后修改url覆盖其他子域名会更好
  // 直接用https://.bilibili.com是不行的
  // {
  //   "url":"https://www.bilibili.com",
  //   "cookies":"a=abcdefg; b=hijklmn"
  // }
]
```

### 制作session

创建session

```rust
let state = SessionState::from_path("./cookies.json").map(Arc::new)?;

let client = ClientBuilder::new()
    .cookie_provider(state.store.clone())//需要启动reqwest -F cookies
    .default_headers(HEADER.clone())
    .build()?;

let session = Session::new(client, state);
```

制备密钥

```rust
session.refresh_csrf(); // 制备csrf，即bili_jct

let url = NavQuery::new().to_query()?.to_url(NAV_URL);

let mixin_key = session
    .get(url)
    .send()
    .await?
    .json::<BiliResponse<Nav>>()
    .await
    .unwrap()
    .data()?
    .wbi_img
    .mixin_key();
session.set_mixin_key(&mixin_key); // 制备sign
```

### 下载视频

```rust
#[tokio::test]
#[ignore]
async fn test_download_video(){
    const BVID:&str = "BV1RHMgz4EnY";

    let url = VideoCidsQuery::from(BVID)
        .to_query()?
        .to_url(VIDEO_CIDS_URL);

    let cid = session
        .get(&url)
        .send()
        .await?
        .json::<BiliResponse<VideoCids>>()
        .await
        .unwrap()
        .data()?
        .get(0)
        .ok_or(Error::msg("cid"))?
        .cid;

    let url = VideoStreamQuery::builder()
        .cid(cid)
        .fnval(Fnval::DASH)
        .vid(VideoQuery::from(BVID))
        .build()
        .to_query()?
        .with_sign(&session.mixin_key())?
        .to_url(VIDEO_STREAM_URL);

    let stream = session
        .get(url)
        .send()
        .await?
        .json::<BiliResponse<VideoStream>>()
        .await?
        .data()?;
    if let (Some(video), Some(audio)) = stream.dash.get_best() {
        (video.base_url.clone(), audio.base_url.clone())
        
        todo!("获得流，自行下载和合并")
    }
    
    session.save_cookies("./cookies_v2.json");
}

```

### 一般流程

先前版本用于省略模板代码的宏被发现并没有使使用变轻松，
并且对库的制作增加了很多工作量，现已删除

请求的基本流程如下

1. new XXXQuery -> query  //对于参数大于3的Query,使用builder
2. url = query.to_query()?.with_sign/with_csrf()?.to_url(URL);//如无需加密则省去sign/csrf
3. 发起请求
4. 解释响应体json(BiliResponse<XXX>)
5. 获得数据(response.data()) // 存在请求成功而data为空的情况，此时response.data()会产生NullResponseDataError

```rust
const BVID: &str = "BV1wDCwYfE2f";

#[tokio::test]
async fn get_video_desc(session:&Session) {
  let url = VideoDescQuery::from(BVID)
      .to_query()
      .unwrap()
      .to_url(VIDEO_DESC_URL);

  let json = session.get(url).send().await.unwrap().text().await.unwrap();

  fs::write("../tests/datas/video_desc.json", &json)
      .await
      .unwrap();

  let desc = serde_json::from_str::<BiliResponse<VideoDesc>>(&json)
      .unwrap()
      .data()
      .unwrap();

  assert!(desc.starts_with("「あたしはまた弱虫モンブランだったみたいだ」"));
}
```

## workspace

理论上支持wasm,还没测试

## 进度

上游挂了

## 共同建设
