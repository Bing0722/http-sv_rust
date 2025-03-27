//! request 请求

use crate::headers::{
    Headers, HttpHeaders, HttpMethod, HttpVersion, IntoHttpMethod, IntoHttpVersion, read_headers,
};

#[allow(unused)]
pub trait IntoRequest {
    fn into_request(&self) -> Request;
}

#[derive(Debug)]
pub struct Request {
    pub start_line: StartLine,
    pub headers: HttpHeaders,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new() -> Self {
        Self {
            start_line: StartLine {
                method: HttpMethod::default(),
                path: "/".to_string(),
                version: HttpVersion::default(),
            },
            headers: HttpHeaders::default(),
            body: Vec::new(),
        }
    }

    pub fn method(mut self, method: impl IntoHttpMethod) -> Self {
        self.start_line.method = method.into_http_method();
        self
    }

    pub fn method_ref(&self) -> &HttpMethod {
        &self.start_line.method
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.start_line.path = path.into();
        self
    }

    pub fn path_ref(&self) -> &str {
        &self.start_line.path
    }

    pub fn version(mut self, version: impl IntoHttpVersion) -> Self {
        self.start_line.version = version.into_http_version();
        self
    }

    pub fn version_ref(&self) -> &HttpVersion {
        &self.start_line.version
    }

    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self.body.extend_from_slice(b"\r\n");
        self.headers.0.insert(
            Headers::ContentLength.to_string(),
            self.body.len().to_string(),
        );
        self
    }

    pub fn body_ref(&self) -> &Vec<u8> {
        &self.body
    }

    pub fn headers(mut self, headers: Headers, value: impl Into<String>) -> Self {
        self.headers.0.insert(headers.to_string(), value.into());
        self
    }
}

impl Into<Vec<u8>> for Request {
    fn into(self) -> Vec<u8> {
        let mut vec = Vec::new();
        let method: Vec<u8> = self.start_line.method.into();
        let path: Vec<u8> = self.start_line.path.into();
        let version: Vec<u8> = self.start_line.version.into();
        let headers = read_headers(&self.headers.0);
        vec.extend_from_slice(&method);
        vec.extend_from_slice(b" ");
        vec.extend_from_slice(&path);
        vec.extend_from_slice(b" ");
        vec.extend_from_slice(&version);
        vec.extend_from_slice(b"\r\n");
        vec.extend_from_slice(&headers);
        vec.extend_from_slice(b"\r\n");
        vec.extend_from_slice(&self.body);
        vec
    }
}

#[derive(Debug)]
pub struct StartLine {
    pub method: HttpMethod,
    pub path: String,
    pub version: HttpVersion,
}

impl std::fmt::Display for StartLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.method.to_string(),
            self.path,
            self.version.to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Request;

    #[test]
    fn test1() {
        let req = Request::new();
        let v: Vec<u8> = req.into();
        println!("`{}`", String::from_utf8_lossy(&v));
    }
}
