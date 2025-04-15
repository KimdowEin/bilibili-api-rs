# TODO

## 2.0计划

- [x] 构建请求宏
- [x] 重构video info
- [x] 重构video stream
- [x] 重构video action
- [] 补完video
- [] 用户信息
- [] 直播信息

## 2.0目标

1. 取消结构的复用,文档是什么就是什么
2. 更清晰的自定义error
3. 完成user模块
4. api脱离session
5. 请求体使用derive


## 规定

直接响应体

 #[derive(Debug,Clone,PartialEq, Deserialize, Serialize,Data)]

间接响应体

 #[derive(Debug,Clone,PartialEq, Deserialize, Serialize)]

请求体

 #[derive(Debug,Clone,PartialEq, Deserialize, Serialize,Query)]
 impl new()