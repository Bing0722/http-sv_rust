#![allow(unused)]

use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug)]
// 表示该枚举可能会在未来添加新的变体，阻止其他代码直接匹配所有变体
#[non_exhaustive]
pub(crate) enum HttpVersion {
    V1_0,
    V1_1,
    V2,
}

#[derive(Debug)]
// 表示该枚举可能会在未来添加新的变体，阻止其他代码直接匹配所有变体
#[non_exhaustive]
pub enum HttpMethod {
    Get,
}

#[derive(Debug)]
// 表示该枚举可能会在未来添加新的变体，阻止其他代码直接匹配所有变体
#[non_exhaustive]
pub enum StatusCode {
    Ok = 200,
    NotFound = 404,
}

#[derive(Debug)]
// 表示该枚举可能会在未来添加新的变体，阻止其他代码直接匹配所有变体
#[non_exhaustive]
pub enum StatusText {
    Ok,
    NotFound,
}

#[derive(Debug)]
pub struct HttpHeaders(pub(crate) HashMap<String, String>);

#[derive(Debug)]
// 表示该枚举可能会在未来添加新的变体，阻止其他代码直接匹配所有变体
#[non_exhaustive]
pub enum Mime {
    ApplicationJson,
    TextPlain,
}

#[derive(Debug)]
pub struct Time {
    pub(crate) now: DateTime<Utc>,
}

impl Time {
    pub fn new() -> Self {
        Self {
            now: chrono::Utc::now(),
        }
    }
}
