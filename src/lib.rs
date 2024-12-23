

pub mod query;
pub mod model;

#[cfg(feature = "session")]
pub mod service;

pub mod traits;
pub mod error;



















































// //! b站api封装
// //! 分为 用户、视频、登录、签名、错误、接口
// //! 用户模块按区块和查询划分
// //! 如果返回信息组合，则算是区块
// //! 比如导航栏信息，个人空间信息
// //! 如果返回特定信息如硬币数，则算是查询
// //! 


// // 接口
// pub mod common;
// // 错误
// pub mod error;
// // 登录相关
// pub mod login;
// // 签名
// pub mod sign;
// // 用户相关
// pub mod user;
// // 视频相关
// pub mod video;


