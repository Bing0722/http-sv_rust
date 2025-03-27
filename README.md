# 设计思路

v1 简单的 解析请求和发送响应 单线程

v2 泛型

思路： 每一个请求连接都是一个 stream ---> 封装一个 stream

上层抽象

```rust

// 最上层抽象
http::server(service, addr); --> service 泛型 多肽

Request::builder().version(HttpVersion::V1_1).status(HttpStatus::Ok).body(...);
--> 使用多态

Response 同上

```
