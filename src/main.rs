use std::net::TcpStream;

use tracing::level_filters::LevelFilter;

use http_sv::{request::read_http_request, response::return_response, server::serve};

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_max_level(LevelFilter::TRACE)
        .init();

    serve(service, "127.0.0.1:8080")?;

    Ok(())
}

fn service(stream: &mut TcpStream) -> Result<(), Error> {
    read_http_request(stream).unwrap();
    return_response(stream).unwrap();

    Ok(())
}
