// #![allow(unused)]

use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use tracing::info;

/// serve 简单实现 服务器启动
pub fn serve<S, T>(server: S, addr: T) -> Result<(), Box<dyn std::error::Error>>
where
    S: Fn(&mut TcpStream) -> Result<(), Box<dyn std::error::Error>>,
    T: ToSocketAddrs,
{
    let listener = TcpListener::bind(addr)?;
    info!("服务器已启动... ");
    info!("服务器地址为: {}", listener.local_addr()?);

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            server(&mut stream)?;
        }
        continue;
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {}
}
