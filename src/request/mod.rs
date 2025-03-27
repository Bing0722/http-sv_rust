mod request;

use std::{
    collections::HashMap,
    io::{BufRead, Read},
    net::TcpStream,
};

pub use request::Request;
use request::StartLine;

use crate::{
    error::RequestError,
    headers::HttpHeaders,
    utils::parse::{parse_map, parse_newline, parse_separator, parse_space},
};

pub fn handle_request(stream: &mut TcpStream) -> Result<Request, RequestError> {
    let mut buf = [0; 4096];
    let n = match stream.read(&mut buf) {
        Ok(n) => {
            if n == 0 {
                return Err(RequestError::EmptyRequest);
            } else {
                n
            }
        }
        Err(_) => return Err(RequestError::ReadRequestErr),
    };

    let (header, body) = parse_separator(&buf[..n])?;
    let (start_line, header_buf) = parse_newline(header)?;
    let (method, other) = parse_space(start_line)?;
    let (path, version) = parse_space(other)?;
    let mut headers = HashMap::new();
    for line in header_buf.lines() {
        match line {
            Ok(line) => {
                let (key, value) = parse_map(line.as_bytes())?;
                headers.insert(
                    String::from_utf8_lossy(key).to_string(),
                    String::from_utf8_lossy(value).to_string(),
                );
            }
            Err(_) => continue,
        }
    }

    let start_line = StartLine {
        method: method.into(),
        path: String::from_utf8_lossy(path).to_string(),
        version: version.into(),
    };

    let headers = HttpHeaders(headers);

    let req = Request {
        start_line,
        headers,
        body: body.to_vec(),
    };

    Ok(req)
}
