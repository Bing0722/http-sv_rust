#![allow(unused)]

use std::{collections::HashMap, hash::Hash};

use nom::AsBytes;

use crate::{
    body::Body,
    utils::{HttpHeaders, HttpVersion, Mime, StatusCode, StatusText, Time},
};

#[derive(Debug)]
pub struct HttpResponse<B> {
    pub(crate) status_line: StatusLine,
    pub(crate) headers: HttpResponseHeader,
    pub(crate) body: Body<B>,
}

impl<'a, B> HttpResponse<B>
where
    B: AsRef<[u8]>,
{
    pub fn new() -> Self {
        Self {
            status_line: StatusLine::new(),
            headers: HttpResponseHeader::new(),
            body: Body::new(),
        }
    }

    pub fn serialize(&mut self) -> Vec<u8> {
        self.update_content_length();
        [
            self.status_line.to_string().as_bytes(),
            self.headers.to_string().as_bytes(),
            self.body.serialize().as_bytes(),
        ]
        .concat()
    }

    fn update_content_length(&mut self) {
        map_insert(
            &mut self.headers.headers.0,
            "Content-Length".to_owned(),
            self.body.len().to_string(),
        );
    }
}

#[derive(Debug)]
pub struct StatusLine {
    version: String, // 根据请求获取
    status_code: u32,
    status_text: String,
}

impl StatusLine {
    // 默认
    pub fn new() -> Self {
        Self {
            version: "HTTP/1.1".to_owned(),
            status_code: 200,
            status_text: "OK".to_owned(),
        }
    }

    fn set_version(&mut self, version: HttpVersion) {
        match version {
            HttpVersion::V1_0 => self.version = "HTTP/1.0".to_owned(),
            HttpVersion::V1_1 => self.version = "HTTP/1.0".to_owned(),
            HttpVersion::V2 => self.version = "HTTP/2".to_owned(),
        }
    }

    fn set_status_code(&mut self, code: StatusCode) {
        match code {
            StatusCode::Ok => self.status_code = 200,
            StatusCode::NotFound => self.status_code = 404,
        }
    }

    fn set_status_text(&mut self, text: StatusText) {
        match text {
            StatusText::Ok => self.status_text = "OK".to_owned(),
            StatusText::NotFound => self.status_text = "Not Found".to_owned(),
        }
    }
}

impl std::fmt::Display for StatusLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}\r\n",
            self.version, self.status_code, self.status_text
        )
    }
}

#[derive(Debug)]
pub struct HttpResponseHeader {
    headers: HttpHeaders,
    // date: String, // 时间 (自动获取)
    // server: String,       // 服务名称
    // content_length: usize, // 传输长度 (自动获取)
    // content_type: String, // MIME 类型
    // connection: String,   // 是否长连接
    // user_agent: String,   // 用户代理
    // host: String, // host (自动获取)
    // others: HttpHeaders,  // 其他类型
}

#[derive(Debug)]
pub enum Connection {
    KeepAlive,
    Close,
}

impl HttpResponseHeader {
    fn new() -> Self {
        let now = Time::new().now.to_rfc2822().replace("+0000", "GMT");
        let mut headers = HttpHeaders(HashMap::<String, String>::new());
        map_insert(
            &mut headers.0,
            "Host".to_owned(),
            "127.0.0.1:8080".to_owned(),
        );
        map_insert(&mut headers.0, "date".to_owned(), now);
        map_insert(
            &mut headers.0,
            "Server".to_owned(),
            "BingServer/Linux".to_owned(),
        );
        map_insert(
            &mut headers.0,
            "User-Agent".to_owned(),
            "BingServer/1.0".to_owned(),
        );
        map_insert(&mut headers.0, "Content-Length".to_owned(), "0".to_owned());
        map_insert(
            &mut headers.0,
            "Content-Type".to_owned(),
            "text/plain".to_owned(),
        );
        map_insert(&mut headers.0, "Connection".to_owned(), "close".to_owned());
        map_insert(
            &mut headers.0,
            "Host".to_owned(),
            "127.0.0.1:8080".to_owned(),
        );
        Self { headers }
    }
}

// 插入
fn map_insert<K, V>(map: &mut HashMap<K, V>, k: K, v: V)
where
    K: std::cmp::Eq,
    K: Hash,
{
    if let None = map.insert(k, v) {}
}

// 实现ToString
impl std::fmt::Display for HttpResponseHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in &self.headers.0 {
            write!(f, "{}: {}\r\n", k, v);
        }
        write!(f, "\r\n")
    }
}

#[cfg(test)]
mod tests {
    use super::{HttpResponseHeader, StatusLine};

    #[test]
    fn hreader_to_string() {
        let headers = HttpResponseHeader::new().to_string();
        println!("`{}`", headers);
    }

    #[test]
    fn status_line_to_string() {
        let statu_line = StatusLine::new();
        let status_line = format!(
            "{} {} {}",
            statu_line.version, statu_line.status_code, statu_line.status_text
        );

        println!("`{}`", status_line);
    }
}
