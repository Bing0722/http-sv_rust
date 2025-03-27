use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HttpHeaders(pub HashMap<String, String>);

impl HttpHeaders {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn notify(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }
}

impl Default for HttpHeaders {
    fn default() -> Self {
        let mut map = HashMap::new();
        let time = chrono::Utc::now().to_rfc2822().replace("+0000", "GMT");
        map.insert(Headers::Date.to_string(), time);
        let host = "127.0.0.1:8080".to_string();
        map.insert(Headers::Host.to_string(), host);
        map.insert(Headers::ContentLength.to_string(), "0".to_string());
        map.insert(
            Headers::ContentType.to_string(),
            Mime::TextPlain.to_string(),
        );
        map.insert(Headers::Connection.to_string(), "close".to_string());

        Self(map)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
// 表示该枚举可能会在未来添加新的变体，阻止其他代码直接匹配所有变体
#[non_exhaustive]
pub enum Headers {
    Date,
    Host,
    ContentType,
    ContentLength,
    Connection,
    // ....
}

impl ToString for Headers {
    fn to_string(&self) -> String {
        match self {
            Self::Date => "Date".to_string(),
            Self::Host => "Host".to_string(),
            Self::ContentType => "Content-Type".to_string(),
            Self::ContentLength => "Content-Length".to_string(),
            Self::Connection => "Connection".to_string(),
        }
    }
}

#[allow(unused)]
#[derive(Debug, Hash)]
// 表示该枚举可能会在未来添加新的变体，阻止其他代码直接匹配所有变体
#[non_exhaustive]
pub enum Mime {
    TextPlain,
    ApplicationJson,
    // ...
}

impl ToString for Mime {
    fn to_string(&self) -> String {
        match self {
            Self::TextPlain => "text/plain".to_string(),
            Self::ApplicationJson => "application/Json".to_string(),
        }
    }
}

pub(crate) fn read_headers(input: &HashMap<String, String>) -> Vec<u8> {
    let mut vec = Vec::new();
    for (k, v) in input {
        vec.extend_from_slice(k.as_bytes());
        vec.extend_from_slice(b": ");
        vec.extend_from_slice(v.as_bytes());
        vec.extend_from_slice(b"\r\n");
    }
    vec
}
