pub trait IntoHttpVersion {
    fn into_http_version(self) -> HttpVersion;
}

#[derive(Debug, Clone)]
// 表示该枚举可能会在未来添加新的变体，阻止其他代码直接匹配所有变体
#[non_exhaustive]
pub enum HttpVersion {
    V1_0,
    V1_1,
    V2,
    // ....
}

impl Default for HttpVersion {
    fn default() -> Self {
        Self::V1_1
    }
}

impl IntoHttpVersion for &str {
    fn into_http_version(self) -> HttpVersion {
        match self {
            "HTTP/1.0" => HttpVersion::V1_0,
            "HTTP/1.1" => HttpVersion::V1_1,
            "HTTP/2" => HttpVersion::V2,
            _ => HttpVersion::V1_1,
        }
    }
}

impl IntoHttpVersion for HttpVersion {
    fn into_http_version(self) -> HttpVersion {
        self
    }
}

impl From<&[u8]> for HttpVersion {
    fn from(value: &[u8]) -> Self {
        match value {
            b"HTTP/1.0" => Self::V1_0,
            b"HTTP/1.1" => Self::V1_1,
            b"HTTP/2" => Self::V2,
            _ => Self::V1_1,
        }
    }
}

impl Into<Vec<u8>> for HttpVersion {
    fn into(self) -> Vec<u8> {
        match self {
            Self::V1_0 => Vec::from(b"HTTP/1.0"),
            Self::V1_1 => Vec::from(b"HTTP/1.1"),
            Self::V2 => Vec::from(b"HTTP/2"),
        }
    }
}

impl ToString for HttpVersion {
    fn to_string(&self) -> String {
        match self {
            Self::V1_0 => String::from("HTTP/1.0"),
            Self::V1_1 => String::from("HTTP/1.1"),
            Self::V2 => String::from("HTTP/2"),
        }
    }
}
