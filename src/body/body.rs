//!  暂时弃用

pub trait Body {
    fn into_body(self) -> Vec<u8>;
}

impl Body for String {
    fn into_body(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl Body for &str {
    fn into_body(self) -> Vec<u8> {
        self.to_string().into_body()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_body_new() {}
}
