use std::net::TcpListener;

use http_sv::{Request, Router, headers::HttpMethod, response::Response, serve};
use tracing::{info, level_filters::LevelFilter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .with_line_number(true)
        .init();

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    info!("服务器启动.....");
    info!("开始监听： {}", listener.local_addr()?);

    let app = Router::new()
        .route("/", HttpMethod::GET, "Hello, World!".to_string())
        .route("/hello", "get", "Hello, This is a test".to_string())
        .route("/post", "POST", hello)
        .route("/", "POST", |_| "Hello, Rust");

    serve(listener, app);

    Ok(())
}

fn hello(req: Request) -> Response {
    let _ = req;
    Response::new().body("Hello, World".into())
}
