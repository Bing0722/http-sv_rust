pub trait IntoStatusCode {
    fn into_status_code(self) -> StatusCode;
}

#[derive(Debug, Clone)]
// 表示该枚举可能会在未来添加新的变体，阻止其他代码直接匹配所有变体
#[non_exhaustive]
pub enum StatusCode {
    OK,
    NotFound,
    // ....
}

impl IntoStatusCode for StatusCode {
    fn into_status_code(self) -> StatusCode {
        self
    }
}

impl IntoStatusCode for &str {
    fn into_status_code(self) -> StatusCode {
        match self {
            "OK" | "Ok" | "ok" | "oK" => StatusCode::OK,
            "NotFound" => StatusCode::NotFound,
            _ => StatusCode::NotFound,
        }
    }
}

impl IntoStatusCode for String {
    fn into_status_code(self) -> StatusCode {
        self.as_str().into_status_code()
    }
}

impl IntoStatusCode for &String {
    fn into_status_code(self) -> StatusCode {
        self.as_str().into_status_code()
    }
}

impl IntoStatusCode for u128 {
    fn into_status_code(self) -> StatusCode {
        match self {
            200 => StatusCode::OK,
            404 => StatusCode::NotFound,
            _ => StatusCode::NotFound,
        }
    }
}

impl IntoStatusCode for u64 {
    fn into_status_code(self) -> StatusCode {
        (self as u128).into_status_code()
    }
}

impl IntoStatusCode for u32 {
    fn into_status_code(self) -> StatusCode {
        (self as u128).into_status_code()
    }
}

impl IntoStatusCode for u16 {
    fn into_status_code(self) -> StatusCode {
        (self as u128).into_status_code()
    }
}

impl Into<Vec<u8>> for StatusCode {
    fn into(self) -> Vec<u8> {
        match self {
            Self::OK => Vec::from(b"200 OK"),
            Self::NotFound => Vec::from(b"404 Not Found"),
        }
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        Self::OK
    }
}
