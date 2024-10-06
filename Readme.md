### 介绍
对b站api的rust封装(建设中)，基于 [bilibili-API-collect](https://github.com/SocialSisterYi/bilibili-API-collect)

虽然未完成，但已经有💩山的雏形了，不是我的问题



### 使用
```toml
# Cargo.toml
[dependencies]
bilibili-api-rs = {git = "https://github.com/KimdowEin/bilibili-api-rs}
```

### 功能 feature
- "session" 提供一些reqwest模板代码，没写完也没什么用
- "manual" 用于手工过登录人机验证，wasm环境下不可用

### 进度
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
  - [ ] 登录基本信息
  - [ ] 个人中心
  - [ ] 注销登录
  - [ ] 登录记录
  - [ ] Web 端 Cookie 刷新
- [ ] 视频
  - [x] 视频分区一览 (分区代码)
  - [x] 基本信息
  - [ ] 状态数
  - [ ] 快照
  - [ ] 点赞 & 投币 & 收藏 & 分享
  - [ ] TAG
  - [ ] 视频推荐
  - [x] 播放&下载地址 (视频流)
  - [ ] 互动视频
  - [ ] 高能进度条
  - [ ] 信息上报 (心跳及记录历史)
  - [ ] 视频属性数据
  - [ ] 视频在线人数
  - [ ] 视频AI摘要
  - [ ] 稿件投诉
  - [ ] 视频状态数
  - [ ] 视频合集

初步测试
可以完成账号密码登录，获取视频信息并完成下载(对大部分人来说够用了)

### 共同建设
存在一些封装不全,没有pub,结构不合理的地方,欢迎提交pr或issue

