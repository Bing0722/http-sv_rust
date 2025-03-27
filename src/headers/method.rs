pub trait IntoHttpMethod {
    fn into_http_method(self) -> HttpMethod;
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
// 表示该枚举可能会在未来添加新的变体，阻止其他代码直接匹配所有变体
#[non_exhaustive]
pub enum HttpMethod {
    GET,
    POST,
    // ....
}

impl Default for HttpMethod {
    fn default() -> Self {
        Self::GET
    }
}

impl IntoHttpMethod for &str {
    fn into_http_method(self) -> HttpMethod {
        match self {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            _ => HttpMethod::GET,
        }
    }
}

impl IntoHttpMethod for String {
    fn into_http_method(self) -> HttpMethod {
        self.as_str().into_http_method()
    }
}

impl IntoHttpMethod for &String {
    fn into_http_method(self) -> HttpMethod {
        self.as_str().into_http_method()
    }
}

impl IntoHttpMethod for HttpMethod {
    fn into_http_method(self) -> HttpMethod {
        self
    }
}

impl Into<Vec<u8>> for HttpMethod {
    fn into(self) -> Vec<u8> {
        match self {
            Self::GET => Vec::from(b"GET"),
            Self::POST => Vec::from(b"POST"),
        }
    }
}

impl From<&[u8]> for HttpMethod {
    fn from(value: &[u8]) -> Self {
        match value {
            b"GET" => Self::GET,
            b"POST" => Self::POST,
            _ => Self::GET,
        }
    }
}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match self {
            Self::GET => String::from("GET"),
            Self::POST => String::from("POST"),
        }
    }
}
