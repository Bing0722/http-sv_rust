use crate::headers::{
    Headers, HttpHeaders, HttpVersion, IntoHttpVersion, IntoStatusCode, StatusCode, read_headers,
};

#[allow(unused)]
pub trait IntoResponse {
    fn into_response(&self) -> Response;
}

impl IntoResponse for Response {
    fn into_response(&self) -> Response {
        (*self).clone()
    }
}

impl IntoResponse for String {
    fn into_response(&self) -> Response {
        Response::new().body(self.as_bytes().to_vec())
    }
}

impl IntoResponse for &String {
    fn into_response(&self) -> Response {
        Response::new().body(self.as_bytes().to_vec())
    }
}

impl IntoResponse for &str {
    fn into_response(&self) -> Response {
        Response::new().body(self.as_bytes().to_vec())
    }
}

#[derive(Clone)]
pub struct Response {
    status_line: StatusLine,
    headers: HttpHeaders,
    body: Vec<u8>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status_line: StatusLine {
                version: HttpVersion::V1_1,
                status: StatusCode::OK,
            },
            headers: HttpHeaders::default(),
            body: Vec::new(),
        }
    }

    pub fn not_found() -> Self {
        Self {
            status_line: StatusLine {
                version: HttpVersion::V1_1,
                status: StatusCode::NotFound,
            },
            headers: HttpHeaders::default(),
            body: Vec::new(),
        }
    }

    /// 设置响应版本
    pub fn version(mut self, version: impl IntoHttpVersion) -> Self {
        self.status_line.version = version.into_http_version();
        self
    }

    /// 设置响应状态码
    pub fn status(mut self, status: impl IntoStatusCode) -> Self {
        self.status_line.status = status.into_status_code();
        self
    }

    /// 设置响应主体
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self.body.extend_from_slice(b"\r\n");
        self.headers.0.insert(
            Headers::ContentLength.to_string(),
            self.body.len().to_string(),
        );
        self
    }

    /// 添加响应头
    pub fn header(mut self, key: Headers, value: &str) -> Self {
        self.headers.0.insert(key.to_string(), value.to_string());
        self
    }
}

impl Into<Vec<u8>> for Response {
    fn into(self) -> Vec<u8> {
        let mut vec = Vec::new();
        let version: Vec<u8> = self.status_line.version.into();
        let status: Vec<u8> = self.status_line.status.into();
        let headers = read_headers(&self.headers.0);
        vec.extend_from_slice(&version);
        vec.extend_from_slice(b" ");
        vec.extend_from_slice(&status);
        vec.extend_from_slice(b"\r\n");
        vec.extend_from_slice(&headers);
        vec.extend_from_slice(b"\r\n");
        vec.extend_from_slice(&self.body);
        vec
    }
}

#[derive(Clone)]
pub struct StatusLine {
    version: HttpVersion,
    status: StatusCode,
}

#[cfg(test)]
mod tests {}
