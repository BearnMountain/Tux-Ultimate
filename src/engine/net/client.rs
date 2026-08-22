use anyhow::Result;
use tokio::net::{ToSocketAddrs, UdpSocket};

use crate::util::config::Config;

pub struct Client {
    socket: UdpSocket,
}

impl Client {
    /// Connects to server via:
    /// - "localhost:$port"
    /// - "$ip:$port"
    pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<Self> {
        let local_port = format!(
            "localhost:{}", 
            &Config::get().read().unwrap().network.server_port
        );

        let socket = UdpSocket::bind(local_port).await?;
        socket.connect(addr).await?;

        return Ok(Client {
            socket,
        });
    }

    pub async fn send() {}
    pub async fn recieve() {}
}
