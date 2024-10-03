### 介绍
对b站api的rust封装(建设中)，基于 [bilibili-API-collect](https://github.com/SocialSisterYi/bilibili-API-collect)

### 使用
```toml
# Cargo.toml
[dependencies]
bilibili-api-rs = {git = "https://github.com/KimdowEin/bilibili-api-rs}
```

### 功能 feature
- "session" 提供一些reqwest模板代码，wasm环境下不可用
- "manual" 提供人类验证页面的跳转函数，wasm环境下不可用

### 最新进度
初步测试
可以完成账号密码登录，获取视频信息并完成下载(对大部分人来说够用了)

### 共同建设
存在一些封装不全,没有pub,结构不合理的地方,欢迎提交pr或issue

