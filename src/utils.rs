#![allow(unused)]

use std::{collections::HashMap, hash::Hash};

use chrono::{DateTime, Utc};

#[derive(Debug)]
// 表示该枚举可能会在未来添加新的变体，阻止其他代码直接匹配所有变体
#[non_exhaustive]
pub(crate) enum HttpVersion {
    V1_0,
    V1_1,
    V2,
}

impl ToString for HttpVersion {
    fn to_string(&self) -> String {
        match self {
            HttpVersion::V1_0 => "HTTP/1.0".to_string(),
            HttpVersion::V1_1 => "HTTP/1.1".to_string(),
            HttpVersion::V2 => "HTTP/2".to_string(),
        }
    }
}

#[derive(Debug)]
// 表示该枚举可能会在未来添加新的变体，阻止其他代码直接匹配所有变体
#[non_exhaustive]
pub enum HttpMethod {
    Get,
    Post,
    // ...
}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match self {
            HttpMethod::Get => "GET".to_string(),
            HttpMethod::Post => "POST".to_string(),
        }
    }
}

#[derive(Debug)]
// 表示该枚举可能会在未来添加新的变体，阻止其他代码直接匹配所有变体
#[non_exhaustive]
pub enum StatusCode {
    Ok,
    NotFound,
    // ...
}

impl ToString for StatusCode {
    fn to_string(&self) -> String {
        match self {
            StatusCode::Ok => "200".to_string(),
            StatusCode::NotFound => "404".to_string(),
        }
    }
}

#[derive(Debug)]
// 表示该枚举可能会在未来添加新的变体，阻止其他代码直接匹配所有变体
#[non_exhaustive]
pub enum StatusText {
    Ok,
    NotFound,
    // ...
}

impl ToString for StatusText {
    fn to_string(&self) -> String {
        match self {
            StatusText::Ok => "OK".to_string(),
            StatusText::NotFound => "NotFound".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct HttpHeaders(pub(crate) HashMap<String, String>);

impl HttpHeaders {
    pub(crate) fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn set(&mut self, header: HeaderFiled, value: String) {
        map_insert(&mut self.0, header.to_string(), value);
    }

    pub(crate) fn serialize(&mut self) -> Vec<u8> {
        let mut vec: Vec<u8> = vec![];
        for (k, v) in &self.0 {
            vec.extend_from_slice(k.as_bytes());
            vec.extend_from_slice(b": ");
            vec.extend_from_slice(v.as_bytes());
            vec.extend_from_slice(b"\r\n");
        }
        vec.extend_from_slice(b"\r\n");
        vec
    }
}

impl Default for HttpHeaders {
    fn default() -> Self {
        let mut this = HttpHeaders::new();
        this.set(HeaderFiled::Date, Time::now());
        this.set(HeaderFiled::Host, "127.0.0.1:8080".to_string());
        this.set(HeaderFiled::ContentLength, "0".to_string());

        this
    }
}

// 插入
pub(crate) fn map_insert<K, V>(map: &mut HashMap<K, V>, k: K, v: V)
where
    K: std::cmp::Eq,
    K: Hash,
{
    if let None = map.insert(k, v) {}
}

#[derive(Debug)]
// 表示该枚举可能会在未来添加新的变体，阻止其他代码直接匹配所有变体
#[non_exhaustive]
pub enum HeaderFiled {
    ContentType,
    ContentLength,
    Connection,
    Date,
    Host,
    Server,
    UserAgent,
    // ...
}

impl ToString for HeaderFiled {
    fn to_string(&self) -> String {
        match self {
            HeaderFiled::ContentType => "Content-Type".to_string(),
            HeaderFiled::ContentLength => "Content-Length".to_string(),
            HeaderFiled::Connection => "Connection".to_string(),
            HeaderFiled::Date => "Date".to_string(),
            HeaderFiled::Host => "Host".to_string(),
            HeaderFiled::Server => "Server".to_string(),
            HeaderFiled::UserAgent => "User-Agent".to_string(),
        }
    }
}

#[derive(Debug)]
// 表示该枚举可能会在未来添加新的变体，阻止其他代码直接匹配所有变体
#[non_exhaustive]
pub enum Mime {
    ApplicationJson,
    TextPlain,
    // ...
}

impl ToString for Mime {
    fn to_string(&self) -> String {
        match self {
            Mime::ApplicationJson => "Application/json".to_string(),
            Mime::TextPlain => "Text/plain".to_string(),
        }
    }
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

    /// 返回 GMT 格式时间
    #[inline]
    pub fn now() -> String {
        chrono::Utc::now().to_rfc2822().replace("+0000", "GMT")
    }
}
