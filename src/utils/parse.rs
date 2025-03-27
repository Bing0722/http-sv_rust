//! 解析

use crate::error::ParseError;
use nom::{
    Err,
    bytes::complete::{tag, take_until},
    error::ErrorKind,
};

pub(crate) const SEPARATOR: &[u8] = b"\r\n\r\n";
pub(crate) const NEWLINE: &[u8] = b"\r\n";
pub(crate) const SPACE: &[u8] = b" ";
pub(crate) const COLON: &[u8] = b": ";

/// 解析分隔符 b"\r\n\r\n"
/// 返回换行符之前和之后的内容
///
/// # Example
/// ```rust
/// const REQUEST: &[u8] = b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\nHello,World\r\n";
/// let (headers, body) = parse_separator(REQUEST).unwrap();
///
/// assert_eq!(headers, b"GET / HTTP/1.1\r\nHost: example.com");
/// assert_eq!(body, b"Hello,World\r\n");
///
/// // body没有的时候
/// REQUEST: &[u8] = b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n";
/// let (headers, body) = parse_separator(REQUEST).unwrap();
///
/// assert_eq!(headers, b"GET / HTTP/1.1\r\nHost: example.com");
/// assert_eq!(body, b"");
/// ```
pub(crate) fn parse_separator(input: &[u8]) -> Result<(&[u8], &[u8]), ParseError> {
    match take_until(SEPARATOR)(input) {
        Ok((second, first)) => Ok((first, tag_consume(second, SEPARATOR)?)),
        Err(Err::Error((_, ErrorKind::TakeUntil))) => Err(ParseError::ParseSeparatorErr),
        _ => Err(ParseError::ParseSeparatorErr),
    }
}

/// 解析新行 b"\r\n"
/// 返回 新行之前和之后的内容
///
/// # Example
/// ```rust
/// const REQUEST: &[u8] = b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\nHello,World\r\n";
/// let (headers, body) = parse_newline(REQUEST).unwrap();
///
/// assert_eq!(headers, b"GET / HTTP/1.1");
/// assert_eq!(body, b"Host: example.com\r\n\r\nHello,World\r\n");
/// ```
pub(crate) fn parse_newline(input: &[u8]) -> Result<(&[u8], &[u8]), ParseError> {
    match take_until(NEWLINE)(input) {
        Ok((second, first)) => Ok((first, tag_consume(second, NEWLINE)?)),
        Err(Err::Error((_, ErrorKind::TakeUntil))) => Err(ParseError::ParseNewlineErr),
        _ => Err(ParseError::ParseNewlineErr),
    }
}

/// 解析空格 b" "
/// 返回空格前后的内容(不包括空格)
///
/// # Example
/// ```rust
/// const REQUEST: &[u8] = b"GET / HTTP/1.1";
/// let (method, other) = parse_space(REQUEST).unwrap();
/// let (path, version) = parse_space(other).unwrap();
///
/// assert_eq!(method, b"GET");
/// assert_eq!(path, b"/");
/// assert_eq!(version, b"HTTP/1.1");
/// ```
pub(crate) fn parse_space(input: &[u8]) -> Result<(&[u8], &[u8]), ParseError> {
    match take_until(SPACE)(input) {
        Ok((second, first)) => Ok((first, tag_consume(second, SPACE)?)),
        Err(Err::Error((_, ErrorKind::TakeUntil))) => Err(ParseError::ParseSpaceErr),
        _ => Err(ParseError::ParseSpaceErr),
    }
}

/// 解析键值对 b": "
/// 返回键值对 用于存储在hashmap中
///
/// # Example
/// ```rust
/// const HEADER: &[u8] = b"Host: example.com\r\nContent-Length: 12\r\nConnection: close\r\n";
/// let mut header: HashMap<String, String> = HashMap::new();
/// for iter in HEADER.lines() {
///     match iter {
///         Ok(i) => {
///             let (k, v) = parse_map(i.as_bytes()).unwrap();
///             header.insert(
///                 String::from_utf8_lossy(k).to_string(),
///                 String::from_utf8_lossy(v).to_string(),
///             );
///         }
///         Err(_) => panic!(""),
///     }
/// }
///
/// assert_eq!(header.get(&"Host".to_string()),Some(&"example.com".to_string()));
/// assert_eq!(header.get(&"Content-Length".to_string()),Some(&"12".to_string()));
/// assert_eq!(header.get(&"Connection".to_string()),Some(&"close".to_string()));
/// ```
pub(crate) fn parse_map(input: &[u8]) -> Result<(&[u8], &[u8]), ParseError> {
    match take_until(COLON)(input) {
        Ok((second, first)) => Ok((first, tag_consume(second, COLON)?)),
        Err(Err::Error((_, ErrorKind::TakeUntil))) => Err(ParseError::ParseMapErr),
        _ => Err(ParseError::ParseMapErr),
    }
}

/// 消耗内容
fn tag_consume<'a>(input: &'a [u8], consume: &'a [u8]) -> Result<&'a [u8], ParseError> {
    match tag(consume)(input) {
        Ok((second, _)) => Ok(second),
        Err(Err::Error((_, ErrorKind::Tag))) => Err(ParseError::ParseConsumeErr),
        _ => Err(ParseError::ParseConsumeErr),
    }
}

#[cfg(test)]
mod tests {

    use std::{collections::HashMap, io::BufRead};

    use crate::utils::parse::{parse_map, parse_newline, parse_separator, parse_space};

    #[test]
    fn test_parse_separator() {
        const REQUEST: &[u8] = b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\nHello,World\r\n";
        let (headers, body) = parse_separator(REQUEST).unwrap();

        assert_eq!(headers, b"GET / HTTP/1.1\r\nHost: example.com");
        assert_eq!(body, b"Hello,World\r\n");
    }

    #[test]
    fn test_parse_separator_no_body() {
        const REQUEST: &[u8] = b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let (headers, body) = parse_separator(REQUEST).unwrap();

        assert_eq!(headers, b"GET / HTTP/1.1\r\nHost: example.com");
        assert_eq!(body, b"");
    }

    #[test]
    fn test_parse_newline() {
        const REQUEST: &[u8] = b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\nHello,World\r\n";
        let (headers, body) = parse_newline(REQUEST).unwrap();

        assert_eq!(headers, b"GET / HTTP/1.1");
        assert_eq!(body, b"Host: example.com\r\n\r\nHello,World\r\n");
    }

    #[test]
    fn test_parse_space() {
        const REQUEST: &[u8] = b"GET / HTTP/1.1";
        let (method, other) = parse_space(REQUEST).unwrap();
        let (path, version) = parse_space(other).unwrap();

        assert_eq!(method, b"GET");
        assert_eq!(path, b"/");
        assert_eq!(version, b"HTTP/1.1");
    }

    #[test]
    fn test_parse_map() {
        const HEADER: &[u8] = b"Host: example.com\r\nContent-Length: 12\r\nConnection: close\r\n";
        let mut header: HashMap<String, String> = HashMap::new();
        for iter in HEADER.lines() {
            match iter {
                Ok(i) => {
                    let (k, v) = parse_map(i.as_bytes()).unwrap();
                    header.insert(
                        String::from_utf8_lossy(k).to_string(),
                        String::from_utf8_lossy(v).to_string(),
                    );
                }
                Err(_) => panic!(""),
            }
        }

        assert_eq!(
            header.get(&"Host".to_string()),
            Some(&"example.com".to_string())
        );
        assert_eq!(
            header.get(&"Content-Length".to_string()),
            Some(&"12".to_string())
        );
        assert_eq!(
            header.get(&"Connection".to_string()),
            Some(&"close".to_string())
        );
    }
}
