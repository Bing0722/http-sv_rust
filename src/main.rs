use std::net::TcpListener;

use tracing::{info, level_filters::LevelFilter};

use http_sv::{request::read_http_request, response::return_response};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_max_level(LevelFilter::INFO)
        .init();

    let server = TcpListener::bind("127.0.0.1:8080")?;
    info!("开始监听： {:?}", server.local_addr().unwrap());

    for stream in server.incoming() {
        if let Ok(mut stream) = stream {
            read_http_request(&mut stream).unwrap();
            return_response(&mut stream).unwrap();
            continue;
        };
    }

    Ok(())
}
