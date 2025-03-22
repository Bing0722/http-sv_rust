// #![allow(unused)]

use std::{
    collections::HashMap,
    io::{BufRead, Read},
    net::TcpStream,
};

use nom::{
    AsBytes,
    bytes::complete::{tag, take_until},
};

use request_util::{HttpRequest, StartLine};

use tracing::{info, trace};

use crate::{
    body::Body,
    error::{HttpMethodError, HttpVersionError, OtherError, RequestError, UriError},
    utils::{HttpHeaders, HttpMethod, HttpVersion},
};

#[path = "request_util.rs"]
mod request_util;

const DELIMITER: &[u8] = b"\r\n\r\n";

pub fn read_http_request(stream: &mut TcpStream) -> Result<HttpRequest<Vec<u8>>, RequestError> {
    let mut buf = [0; 4096];
    let mut header_buf = Vec::new();
    loop {
        let n = stream.read(&mut buf).unwrap();
        header_buf.extend_from_slice(&buf[..n]); // 克隆并添加元素到header_buf中
        // windows方法返回一个迭代器，每次迭代返回一个长度为4的窗口
        // any方法用于判断迭代器中的元素是否满足条件，这里判断是否包含DELIMITER
        if buf[..n].windows(4).any(|w| w == DELIMITER) {
            break;
        }
    }

    // 切割 header 和 body
    // 解析空白行
    let (body, headers_buf) = parse_blankline(header_buf.as_bytes()).unwrap();

    info!("headers_buf: {:?}", String::from_utf8_lossy(headers_buf));
    info!("body: {:?}", String::from_utf8_lossy(body));

    let (other, start_line) = parse_start_line(headers_buf)?;

    let headers = parse_header(other).unwrap();

    // let body = Some(String::from_utf8_lossy(body).to_string());
    let body = parse_body(body)?;

    info!("请求起始行: {}", start_line.to_string());

    Ok(HttpRequest {
        start_line,
        headers,
        body,
    })
}

fn parse_start_line(input: &[u8]) -> Result<(&[u8], StartLine), RequestError> {
    // 解析方法
    let (other, method_bytes) = parse_method(input, b"GET").unwrap();
    let _method = String::from_utf8_lossy(method_bytes).to_string();
    let method = HttpMethod::Get;
    // 解析路径
    let (other, path_bytes) = parse_path(other).unwrap();
    let path = String::from_utf8_lossy(path_bytes).to_string();

    // 解析版本
    let (other, version_bytes) = parse_version(other).unwrap();
    let version = String::from_utf8_lossy(version_bytes).to_string();
    let version = match &version[..] {
        "1.0" => HttpVersion::V1_0,
        "1.1" => HttpVersion::V1_1,
        "2" => HttpVersion::V2,
        _ => {
            return Err(RequestError::InvalidVersion);
        }
    };

    Ok((
        other,
        StartLine {
            method,
            path,
            version,
        },
    ))
}

/// 解析方法
/// ```
/// const Header: &str = "GET / HTTP/1.1";
/// let (other, method_bytes) = parse_method(&Header.as_bytes(), b"GET").unwrap();
/// let method = String::from_utf8_lossy(method_bytes);
/// println!("Method: `{}`", method); // Method: `GET`
/// println!("Other: `{}`", String::from_utf8_lossy(other)); // Other: `/ HTTP/1.1`
///
/// ```
fn parse_method<'a>(
    input: &'a [u8],
    method: &'a [u8],
) -> Result<(&'a [u8], &'a [u8]), HttpMethodError> {
    match tag(method)(input) {
        Ok((other, method)) => match parse_space(other) {
            Ok(other) => {
                trace!(
                    "method: `{}`, other: `{}`",
                    String::from_utf8_lossy(method),
                    String::from_utf8_lossy(other)
                );
                Ok((other, method))
            }
            Err(_) => Err(HttpMethodError::InvalidMethod),
        },
        Err(nom::Err::Error((_, nom::error::ErrorKind::Tag))) => {
            Err(HttpMethodError::InvalidMethod)
        }
        _ => Err(HttpMethodError::InvalidMethod),
    }
}

/// 解析路径
/// ```
/// // other: &[u8] = "/ HTTP/1.1"
/// let (other, path) = parse_path(input).unwrap();
/// let path = String::from_utf8_lossy(path);
/// println!("Path: `{}`", path); // Path: `/`
/// println!("Other: `{}`", String::from_utf8_lossy(other)); // Other: `HTTP/1.1`
///
/// ```
fn parse_path<'a>(input: &'a [u8]) -> Result<(&'a [u8], &'a [u8]), UriError> {
    match take_until(" ")(input) {
        Ok((other, path)) => match parse_space(other) {
            Ok(other) => {
                trace!(
                    "path: `{}`, other: `{}`",
                    String::from_utf8_lossy(path),
                    String::from_utf8_lossy(other)
                );
                Ok((other, path))
            }
            Err(_) => Err(UriError::InvalidUri),
        },
        Err(nom::Err::Error((_, nom::error::ErrorKind::TakeUntil))) => Err(UriError::InvalidUri),
        _ => Err(UriError::InvalidUri),
    }
}

/// 解析HTTP版本
/// ```
/// // other: &[u8] = "HTTP/1.1\r\n"
/// let (other, version) = parse_version(input).unwrap();
/// let version = String::from_utf8_lossy(version);
/// println!("Version: `{}`", version); // Version: `1.1`
///
/// ```
fn parse_version<'a>(input: &'a [u8]) -> Result<(&'a [u8], &'a [u8]), HttpVersionError> {
    match tag("HTTP/")(input) {
        Ok((other, _)) => match parse_newline(other) {
            Ok((other, version)) => {
                trace!(
                    "Version: `{}`, other: `{}`",
                    String::from_utf8_lossy(version),
                    String::from_utf8_lossy(other)
                );
                Ok((other, version))
            }
            Err(_) => Err(HttpVersionError::InvalidVersion),
        },
        Err(nom::Err::Error((_, nom::error::ErrorKind::Tag))) => {
            Err(HttpVersionError::InvalidVersion)
        }
        _ => Err(HttpVersionError::InvalidVersion),
    }
}

/// 解析标头
/// ```
/// // other: &[u8] = "Content-Type: application/json\r\n";
/// let (key_bytes, value_bytes) = parse_header_other(other).unwrap();
/// println!("Key: `{}`", String::from_utf8_lossy(key_bytes)); // Key: `Content-Type`
/// println!("Value: `{}`", String::from_utf8_lossy(value_bytes)); // Value: `application/json`
/// ```
fn parse_header(input: &[u8]) -> Result<HttpHeaders, RequestError> {
    let mut headers: HashMap<String, String> = HashMap::new();
    for line in input.lines() {
        match line {
            Ok(string) => match parse_map(string.as_bytes()) {
                Ok((value, key)) => {
                    trace!(
                        "{}: {}",
                        String::from_utf8_lossy(key),
                        String::from_utf8_lossy(value)
                    );
                    headers.insert(
                        String::from_utf8_lossy(key).to_string(),
                        String::from_utf8_lossy(value).to_string(),
                    );
                }
                Err(_) => {
                    return Err(RequestError::ParseHeaderError);
                }
            },
            _ => continue,
        }
    }
    Ok(HttpHeaders(headers))
}

// 解析空格
fn parse_space<'a>(input: &'a [u8]) -> Result<&'a [u8], OtherError> {
    // tag 匹配开头是否成功, 成功返回 (剩下的部分, 匹配的部分)
    match tag(" ")(input) {
        Ok((other, _)) => Ok(other),
        Err(nom::Err::Error((_, nom::error::ErrorKind::Tag))) => Err(OtherError::ParseSpaceError),
        _ => Err(OtherError::ParseSpaceError),
    }
}

// 解析键值对
fn parse_map<'a>(input: &'a [u8]) -> Result<(&'a [u8], &'a [u8]), OtherError> {
    // take_until 匹配直到指定的字符串，成功返回 (匹配的后一部分(包括匹配的) , 匹配的前一部分)
    match take_until(": ")(input) {
        Ok((other, front)) => match tag(": ")(other) {
            Ok((other, _)) => Ok((other, front)),
            Err(nom::Err::Error((_, nom::error::ErrorKind::Tag))) => Err(OtherError::ParseMapError),
            _ => Err(OtherError::ParseMapError),
        },
        Err(nom::Err::Error((_, nom::error::ErrorKind::Tag))) => Err(OtherError::ParseMapError),
        _ => Err(OtherError::ParseMapError),
    }
}

// 解析 \r\n
fn parse_newline<'a>(input: &'a [u8]) -> Result<(&'a [u8], &'a [u8]), OtherError> {
    match take_until("\r\n")(input) {
        Ok((other, front)) => match tag("\r\n")(other) {
            Ok((other, _)) => Ok((other, front)),
            Err(nom::Err::Error((_, nom::error::ErrorKind::Tag))) => {
                Err(OtherError::ParseNewlineError)
            }
            _ => Err(OtherError::ParseNewlineError),
        },
        Err(nom::Err::Error((_, nom::error::ErrorKind::Tag))) => Err(OtherError::ParseNewlineError),
        _ => Err(OtherError::ParseNewlineError),
    }
}

// 解析空白行
fn parse_blankline(input: &[u8]) -> Result<(&[u8], &[u8]), OtherError> {
    match take_until("\r\n\r\n")(input) {
        Ok((other, front)) => match tag("\r\n\r\n")(other) {
            Ok((other, _)) => Ok((other, front)),
            Err(nom::Err::Error((_, nom::error::ErrorKind::Tag))) => {
                Err(OtherError::ParseBlanklineError)
            }
            _ => Err(OtherError::ParseBlanklineError),
        },
        Err(nom::Err::Error((_, nom::error::ErrorKind::TakeUntil))) => {
            Err(OtherError::ParseBlanklineError)
        }
        _ => Err(OtherError::ParseBlanklineError),
    }
}

/// 解析body
fn parse_body(input: &[u8]) -> Result<Body<Vec<u8>>, RequestError> {
    let mut body: Body<Vec<u8>> = Body::new();
    if input.is_empty() {
        return Ok(body);
    }
    body.set_body(input.to_vec());

    Ok(body)
}
