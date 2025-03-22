use std::{io::Write, net::TcpStream};

use nom::AsBytes;

use crate::error::ResponseError;

use super::response_util::HttpResponse;

pub fn return_response(stream: &mut TcpStream) -> Result<HttpResponse<String>, ResponseError> {
    let mut response: HttpResponse<String> = HttpResponse::new();
    response.body.set_body("Hello, World".to_owned());

    let buf: Vec<u8> = response.serialize();

    println!("{}", String::from_utf8_lossy(buf.as_bytes()));

    stream.write(buf.as_bytes()).unwrap();
    Ok(HttpResponse::<String>::new())
}

#[cfg(test)]
mod tests {

    #[test]
    fn time_test() {}
}
