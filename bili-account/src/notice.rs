// | 字段        | 类型 | 内容     | 备注 |
// | ----------- | ---- | -------- | ---- |
// | id          | num  | id       |      |
// | content     | str  | 显示文案 |      |
// | url         | str  | 跳转地址 |      |
// | notice_type | num  | 提示类型 | 1,2  |
// | icon        | str  | 前缀图标 |      |
// | text_color  | str  | 文字颜色 |      |
// | bg_color    | str  | 背景颜色 |      |

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountNotice {
    pub id: i32,
    pub content: String,
    pub url: String,
    pub notice_type: i32,
    pub icon: String,
    pub text_color: String,
    pub bg_color: String,
}
