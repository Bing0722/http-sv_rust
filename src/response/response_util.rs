#![allow(unused)]

use nom::AsBytes;

use crate::{
    body::Body,
    utils::{HeaderFiled, HttpHeaders, HttpVersion, StatusCode, StatusText},
};

#[derive(Debug)]
pub struct HttpResponse<B> {
    pub(crate) status_line: StatusLine,
    pub(crate) headers: HttpHeaders,
    pub(crate) body: Body<B>,
}

impl<'a, B> HttpResponse<B>
where
    B: AsRef<[u8]>,
{
    pub fn new() -> Self {
        Self {
            status_line: StatusLine::new(),
            headers: HttpHeaders::new(),
            body: Body::new(),
        }
    }

    pub fn serialize(&mut self) -> Vec<u8> {
        // 更新长度
        self.update_content_length();
        [
            self.status_line.serialize().as_bytes(),
            self.headers.serialize().as_bytes(),
            self.body.serialize().as_bytes(),
        ]
        .concat()
    }

    fn update_content_length(&mut self) {
        self.headers
            .set(HeaderFiled::ContentLength, self.body.len().to_string());
    }
}

#[derive(Debug)]
pub struct StatusLine {
    version: String, // 根据请求获取
    status_code: String,
    status_text: String,
}

impl StatusLine {
    // 默认
    pub fn new() -> Self {
        Self {
            version: HttpVersion::V1_1.to_string(),
            status_code: StatusCode::Ok.to_string(),
            status_text: StatusText::Ok.to_string(),
        }
    }

    fn set_version(&mut self, version: HttpVersion) {
        self.version = version.to_string();
    }

    fn set_status_code(&mut self, code: StatusCode) {
        self.status_code = code.to_string();
    }

    fn set_status_text(&mut self, text: StatusText) {
        self.status_text = text.to_string();
    }

    fn serialize(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = vec![];
        vec.extend_from_slice(self.version.as_bytes());
        vec.extend_from_slice(b" ");
        vec.extend_from_slice(self.status_code.as_bytes());
        vec.extend_from_slice(b" ");
        vec.extend_from_slice(self.status_text.as_bytes());
        vec.extend_from_slice(b"\r\n");
        vec
    }
}

#[derive(Debug)]
pub enum Connection {
    KeepAlive,
    Close,
}

#[cfg(test)]
mod tests {
    use nom::AsBytes;

    use crate::utils::HttpHeaders;

    use super::StatusLine;

    #[test]
    fn hreader_to_string() {
        let headers = HttpHeaders::new().serialize();
        println!("`{}`", String::from_utf8_lossy(headers.as_bytes()));
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
