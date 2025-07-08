# bilibili-api-rs

## 声明

本库仅用于学习和测试,禁止用于任何非法用途,灰产或其他恶劣行为.不当使用产生的所有后果与作者无关.

如果你对此库的存在有任何不满,我立马删库致歉,官方打过来我第一个投降.

![投降](投降.jpg)

## 介绍

对b站api的rust封装(建设中)，基于 [bilibili-API-collect](https://github.com/SocialSisterYi/bilibili-API-collect)

会坚持弄完的

v0.3.0建设中,会重构大部分api,并重构部分模块

后续会逐步完成其他不常用的模块,并补充文档

## 快速开始

### 导入库

```toml
# Cargo.toml
[dependencies.bilibili-api-rs]
git = "https://github.com/KimdowEin/bilibili-api-rs"
brance = "night"
features = ["session"]
```

### 登录(password)

```rust

#[tokio::test]
async fn test_login() {
    let (username, password) = ("username", "password");
    let session = Session::new().unwrap();

    let query = CaptchaQuery::new();
    let captcha = CaptchaRequest::send_request(&session, query).await.unwrap();

    // 这里会使用默认浏览器跳转到一个过captcha的页面，需要手动验证
    // 需要启用 feature = manual
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
let session = Session::new_with_path("./cookies.json").unwrap();
```

### 下载视频

```rust
#[tokio::test]
#[ignore]
async fn test_download_video(){
    const BVID:&str = "BV1RHMgz4EnY";

    let session = Session::new_with_path("./cookies.json").unwrap();

    let query = VideoCidsQuery::from(BVID);
    let cids = VideoCidsRequest::send_request(&session, query).await.unwrap();
    let cid = cids[0].cid;

    let vid = VideoQuery::from(BVID);
    let query = VideoStreamQuery::builder()
        .vid(vid)
        .cid(cid)
        .fnval(Fnval::DASH)
        .build();
    let video_stream = VideoStreamRequest::send_request(&session, query).await.unwrap();
    let (video,audio) = video_stream.dash.get_best();
    if let (Some(video),Some(audio)) = (video,audio) {
        let temp_dir = TempDir::new().unwrap();
        
        let video_path = temp_dir.path().join("video");
        let audio_path = temp_dir.path().join("audio");

        let download_video = download_file(&session, &video.base_url, &video_path);
        let download_audio = download_file(&session, &audio.base_url, &audio_path);

        tokio::try_join!(download_video, download_audio).expect("下载失败");

        let output_path = "./tests/output/output.mp4";
        merge_video_audio(&video_path, &audio_path, output_path).await.expect("合并失败");

        // 删除源文件（临时文件会随 temp_dir 被自动删除）
        println!("合并完成，输出文件：{}", output_path);
    }
    
}

async fn download_file(session: &Session, url: &str, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let response = session.get(url).send().await?;
    let total_size = response.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{wide_bar}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let mut file = File::create(path).await?;
    let mut downloaded = 0u64;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("下载完成");
    Ok(())
}

async fn merge_video_audio(video_path: &Path, audio_path: &Path, output_path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let output = tokio::process::Command::new("ffmpeg")
        .arg("-i")
        .arg(video_path)
        .arg("-i")
        .arg(audio_path)
        .arg("-c:v")
        .arg("copy")
        .arg("-c:a")
        .arg("aac")
        .arg("-strict")
        .arg("experimental")
        .arg(output_path)
        .output()
        .await?;

    if !output.status.success() {
        return Err(format!("FFmpeg 合并失败: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    Ok(())
}

```

### 一般流程

理论上，所有接口都有对应模板代码

1. new XXXQuery -> query
2. XXXRequest::send_request(&session,query) -> XXX

如果是非常新的版本，可能还没有创建,则按照如下步骤

1. 找到请求体(XXXQuery),生成请求(query/sign/csrf)
2. 和url(XXX_URL)拼接({}?{},url,query)
3. 发起请求
4. 解释响应体json(BiliResponse<XXX>)
5. 获得数据(response.data())

```rust
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

- "session"
  - 提供一个会话
  - 请求端口
  - cookies的导入和保存
- "manual"
  - 提供一个函数跳转到过人机验证的网站

## 进度

经过几次大改后进度混乱，暂时无法确定各模块完整性。

但具有端口结构的能保证可靠性

## 共同建设

需要大量测试用例

将仓库clone下来,配置好cookies.json,
在tests/tests.toml中添加测试用例,
然后执行cargo test.(还没写好)

将失败的用例提交issue,或者自己修bug

测试代码以后会逐渐变得复杂且耗时

api的更新是很迅速的,如观测到变化,
请提交到[bilibili-API-collect](https://github.com/SocialSisterYi/bilibili-API-collect).
然后@我,我再更新到这个仓库中.
