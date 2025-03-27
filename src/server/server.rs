use std::{
    io::Write,
    net::{SocketAddr, TcpListener, TcpStream},
};

use crate::{
    error::ResponseError,
    headers::Headers,
    request::Request,
    response::{IntoResponse, Response},
    router::Router,
};

use super::Service;

/// Server
pub struct Server {
    listener: TcpListener,
    service: Router,
}

impl Server {
    /// 创建一个服务器
    pub fn new(listener: TcpListener, service: Router) -> Self {
        Self { listener, service }
    }

    /// 获取本地地址
    pub fn local_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.listener.local_addr()
    }

    pub fn start(&mut self) {
        for stream in self.listener.incoming() {
            if let Ok(stream) = stream {
                let remote_addr = stream.peer_addr().unwrap();
                let local_addr = self.local_addr().unwrap();

                let mut incoming_stream = IncomingStream::new(stream, remote_addr);

                let req = self.service.call(&mut incoming_stream).unwrap();
                let resp = self
                    .service
                    .handle(req)
                    .header(Headers::Host, local_addr.to_string().as_str());

                let buf: Vec<u8> = resp.into();
                incoming_stream.stream_mut().write_all(&buf).unwrap();
            }
        }
    }
}

pub struct IncomingStream {
    pub stream: TcpStream,
    pub remote_addr: std::net::SocketAddr,
}

impl IncomingStream {
    pub fn new(stream: TcpStream, remote_addr: SocketAddr) -> Self {
        Self {
            stream,
            remote_addr,
        }
    }

    pub fn remote_addr(&self) -> SocketAddr {
        self.remote_addr
    }

    pub fn stream_mut(&mut self) -> &mut TcpStream {
        &mut self.stream
    }
}

/// 服务启动
pub fn serve(listener: TcpListener, router: Router) {
    let mut server = Server::new(listener, router);
    server.start();
}

impl Service<Request> for String {
    type Response = Response;
    type Error = ResponseError;

    fn call(&mut self, req: Request) -> Result<Self::Response, Self::Error> {
        let _ = req;
        let resp = Response::new().body(self.clone().into_bytes());
        Ok(resp)
    }
}

impl Service<Request> for &str {
    type Response = Response;
    type Error = ResponseError;

    fn call(&mut self, req: Request) -> Result<Self::Response, Self::Error> {
        let _ = req;
        let resp = Response::new().body(self.to_string().into_bytes());
        Ok(resp)
    }
}

/// 为闭包实现服务
impl<F, R> Service<Request> for F
where
    F: FnOnce(Request) -> R,
    F: Copy,
    R: IntoResponse,
{
    type Response = Response;
    type Error = ResponseError;

    fn call(&mut self, req: Request) -> Result<Self::Response, Self::Error> {
        let resp = (self)(req);
        Ok(resp.into_response())
    }
}
