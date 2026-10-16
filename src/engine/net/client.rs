use std::{
    collections::VecDeque, 
    net::SocketAddr, 
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, time::Duration};

use tokio::{net::UdpSocket, task::JoinHandle, time::timeout};

use crate::engine::net::{packet::{ClientId, MAX_PACKET_SIZE, RawPacket}, protocol::{self, PacketType}};

const RETRY_EVERY: Duration = Duration::from_millis(500);
const MAX_RETRIES: usize = 6;
const MAX_INBOX: usize = 4096;
const KEEP_CONNECT_EVERY: Duration = Duration::from_secs(2);

pub struct NetworkClient {
    socket: Arc<UdpSocket>,
    inbox: Arc<Mutex<VecDeque<Vec<u8>>>>,
    connected: Arc<AtomicBool>,
    id: ClientId,
    tasks: Vec<JoinHandle<()>>,
}

/// Handlings continous connection + packet inbox(async recv packets)
/// 
/// Usage:
/// ```
/// 
///
///
///
///
/// ```
impl NetworkClient {
    /// connects to server + multi try
    pub async fn connect(server: &str) -> anyhow::Result<Self> {
        let server: SocketAddr = tokio::net::lookup_host(server)
            .await?
            .next()
            .ok_or_else(|| anyhow::anyhow!("could not result {server}"))?;
        let local = if server.is_ipv4() { "0.0.0.0:0" } else { "[::]:0" };
        let socket = UdpSocket::bind(local).await?;
        socket.connect(server).await?; // binds socket to server
        let socket = Arc::new(socket);

        let hello = protocol::encode(PacketType::CONNECT, &[]);
        let mut buffer = vec![0u8; MAX_PACKET_SIZE + 1];
        let mut id = None;
        'retry: for _ in 0..MAX_RETRIES {
            let _ = socket.send(&hello).await; // error then retry

            let deadline = tokio::time::Instant::now() + RETRY_EVERY;
            loop {
                let time_left = deadline
                    .saturating_duration_since(tokio::time::Instant::now());

                // waits for accept
                match timeout(time_left, socket.recv(&mut buffer)).await {
                    Ok(Ok(psize)) if psize <= MAX_PACKET_SIZE => {
                        if let Some((PacketType::ACCEPT, body)) = 
                            protocol::parse(&buffer[..psize]) 
                        {
                            if body.len() >= 4 {
                                id = Some(u32::from_le_bytes(body[..4].try_into().unwrap()));
                                break 'retry;
                            }
                        }
                    },
                    /// no packet, retry in a few
                    Ok(_) => tokio::time::sleep(Duration::from_millis(20)).await,
                    Err(_) => break, // time out -> resend
                }
            }
        }

        let id = id.ok_or_else(|| anyhow::anyhow!("server didn't respond"))?;

        // connected, now create client 
        let inbox = Arc::new(Mutex::new(VecDeque::new()));
        let connected = Arc::new(AtomicBool::new(true));
        let recv = tokio::spawn(Self::packet_handler(
            socket.clone(),
            inbox.clone(),
            connected.clone(),
        ));
        let keep_alive = tokio::spawn(Self::keep_alive(
            socket.clone(),
            connected.clone(),
        ));

        return Ok(Self {
            socket,
            inbox,
            connected,
            id: id as ClientId,
            tasks: vec![recv, keep_alive],
        });
    }

    /// returns all recieved packets from the server
    pub fn poll(&self) -> Vec<Vec<u8>> {
        return self.inbox.lock().unwrap().drain(..).collect();
    }

    /// non blocking send
    pub fn send(&self, data: &[u8]) -> anyhow::Result<()> {
        anyhow::ensure!(data.len() <= MAX_PACKET_SIZE, "payload too large");
        anyhow::ensure!(self.is_connected(), "not connected");
        self.socket.try_send(&protocol::encode(PacketType::DATA, data))?;
        return Ok(());
    }

    pub fn is_connected(&self) -> bool {
        return self.connected.load(Ordering::Relaxed);
    }

    pub fn disconnect(&self) {
        if self.connected.swap(false, Ordering::Relaxed) {
            let _ = self.socket.try_send(
                &protocol::encode(PacketType::DISCONNECT, &[])
            );
        }
    }

    /// handles all incoming packet for self.inbox
    async fn packet_handler(
        socket: Arc<UdpSocket>,
        inbox: Arc<Mutex<VecDeque<Vec<u8>>>>,
        connected: Arc<AtomicBool>,
    ) {
        let mut buf = vec![0u8; MAX_PACKET_SIZE + 1];
        loop {
            let packet_size = match socket.recv(&mut buf).await {
                Ok(n) => n,
                Err(_) => { // server not set up or other errors
                    // sleeps until more can come in
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    continue;
                }
            };

            // packet isnt from the server
            if packet_size > MAX_PACKET_SIZE {
                continue;
            }

            match protocol::parse(&buf[..packet_size]) {
                Some((PacketType::DATA, body)) => {
                    let mut q = inbox.lock().unwrap();
                    if q.len() < MAX_INBOX {
                        q.push_back(body.to_vec());
                    }
                },
                Some((PacketType::DISCONNECT, _)) => {
                    connected.store(false, Ordering::Relaxed);
                    break;
                },
                _ => {}
            }
        }
    }

    /// ensures connection holds with server, must reconnect if dropped
    async fn keep_alive(socket: Arc<UdpSocket>, connected: Arc<AtomicBool>) {
        let packet = protocol::encode(PacketType::KEEP_ALIVE, &[]);
        let mut tick = tokio::time::interval(KEEP_CONNECT_EVERY);
        while connected.load(Ordering::Relaxed) {
            tick.tick().await;
            let _ = socket.send(&packet).await;
        }
    }
}

/// safely disconnect
impl Drop for NetworkClient {
    fn drop(&mut self) {
        self.disconnect();
        for task in &self.tasks {
            task.abort();
        }
    }
}
