use std::{collections::HashMap};

use tokio::net::{UdpSocket};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

struct ConnectionSlot {
    latest_tick: u64,
    latest_playload: Vec<u8>,
}

pub struct Server {
    max_connections: Arc<u32>,
    socket: Arc<UdpSocket>,
    connections: Arc<Mutex<HashMap<SocketAddr, ConnectionSlot>>>,
}

impl Server {
    pub async fn create(
        port: u32,
        max_connections: u32,
    ) -> anyhow::Result<Self> {
        let socket = UdpSocket::bind(format!("localhost:{}", port)).await?;
        let socket = Arc::new(socket);
        let connections = Arc::new(Mutex::new(HashMap::new()));
        let connections_cap = Arc::new(max_connections);

        // server thread created
        let server_max_connections = Arc::clone(&connections_cap);
        let server_socket = Arc::clone(&socket);
        let server_connections = Arc::clone(&connections);

        std::thread::spawn(move || Self::server_thread(
            server_max_connections, 
            server_socket, 
            server_connections,
        ));
        
        return Ok(Self {
            max_connections: connections_cap,
            socket, 
            connections,
        });
    }

    async fn send_all(&self, data: &[u8]) {
        let conns = self.connections.lock().unwrap();
        for addr in conns.keys() {
            let _ = self.send_to(addr, data).await;
        }
    }
    async fn send_to(
        &self,
        addr: &SocketAddr,
        data: &[u8],
    ) -> std::io::Result<usize> {
        return self.socket.send_to(data, addr).await;
    }

    // Server handling thread:
    // - connections
    // - all authoritative server logic
    // - 
    fn server_thread(
        _max_connections: Arc<u32>,
        _recv_socket: Arc<UdpSocket>, 
        _connections: Arc<Mutex<HashMap<SocketAddr, ConnectionSlot>>>,
    ) {
        let mut buffer: [u8; 1500]; // max ethernet packet size, if get split, rip

        loop {
        }
    }
}
