use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::interval;
use wgpu::wgc::registry;

use crate::engine::net::packet::{ClientId, MAX_PACKET_SIZE, RawPacket};
use crate::engine::net::protocol::{self, PacketType};

/*
Server paradigm: based on Gafferon Games
- Authenticate: 
    - player connect, sends public key in plaintext
    - server creates encrypted challenge with client public key:
        - packet stores challenge bit sequence + server public key
    - client decrypts and resends challenge
    - server authenticates and logs user ip 
- Validate Packets: if ip of packet is logged, tries unencryption
    - raw data must be parseable

Network -> Packet decoder -> session manager -> matchmaking ->
game simulation -> state replication
*/

const MAX_INBOX: usize = 4096; // 2^12 packets max
const MAX_CLIENT_REGISTRY: usize = 256;
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Client tracking info for holding connections
#[derive(Clone, Debug)]
pub struct ClientInfo {
    pub id: ClientId,
    pub addr: SocketAddr,
    pub last_packet: Instant, // time out/lose connection
    // registered_at 
    // recv_packets
}

#[derive(Default)]
struct Registry {
    addr_to_client: HashMap<SocketAddr, ClientId>, 
    clients: HashMap<ClientId, ClientInfo>,
    next_id: ClientId, // max 256 players
}

impl Registry {
    /// Registers new client unless already registered
    fn register(&mut self, addr: SocketAddr) -> Option<ClientId> {
        // already registered
        if let Some(&id) = self.addr_to_client.get(&addr) {
            self.touch(id);
            return Some(id);
        }

        if self.clients.len() >= MAX_CLIENT_REGISTRY {
            return None;
        }

        // inits client info
        self.next_id += 1;
        let id = self.next_id;
        self.addr_to_client.insert(addr, id);
        self.clients.insert(id, ClientInfo {
            id,
            addr,
            last_packet: Instant::now(),
        });

        log::info!("[server] client({id}) connected from: {addr}");
        return Some(id);
    }

    fn touch(&mut self, id: ClientId) {
        // inits client info
        if let Some(client) = self.clients.get_mut(&id) {
            client.last_packet = Instant::now();
        }
    }


    /// check if addr is known
    fn touch_addr(&mut self, addr: &SocketAddr) -> Option<ClientId> {
        let id = *self.by_addr.get(addr)?;
        self.touch(id);
        return Some(id);
    }

    fn remove(&mut self, id: ClientId) -> Option<ClientInfo> {
        let client = self.clients.remove(&id)?;
        self.addr_to_client.remove(&client.addr);
        log::info!("[server] removed client({id})");

        return Some(client);
    }
}

/// Server is a background task manager for handling incoming and outgoing
/// packets and information. Thin layer that:
/// - connects players through an encrypted line
/// - encrypts and decrypts packets
/// - maintains server connection
/// - stores raw packets
///
/// Usage:
/// '''
/// let server = GameServer::bind("0.0.0.0:2760").await?;
/// server.run().await;
/// '''
pub struct NetworkServer {
    socket: Arc<UdpSocket>,
    inbox: Arc<Mutex<Vec<RawPacket>>>,
    registry: Arc<Mutex<Registry>>,
    tasks: Vec<JoinHandle<()>>,
}

impl NetworkServer {
    /// bind server on host computer
    pub async fn cjeate(lan: bool, port: u16) -> anyhow::Result<Self> {
        let host = if lan { "0.0.0.0" } else { "127.0.0.1" };
        let socket = Arc::new(UdpSocket::bind((host, port)).await?);
        log::info!("[server] listening on {:?}", socket.local_addr());

        // handles server connections + packets
        let inbox = Arc::new(Mutex::new(Vec::new()));
        let registry = Arc::new(Mutex::new(Registry::default()));

        // background tasks
        let recv = tokio::spawn(Self::recv_loop(
            socket.clone(),
            inbox.clone(),
            registry.clone(),
        ));
        let kick = tokio::spawn(Self::timeout_loop(
            socket.clone(),
            registry.clone(),
        ));

        return Ok(Self {
            socket,
            inbox: Arc::new(Mutex::new(Vec::new())),
            registry,
            tasks: vec![recv, kick],
        });
    }

    pub fn send(&self, id: ClientId, data: &[u8]) -> anyhow::Result<()> {
        anyhow::ensure!(data.len() <= MAX_PACKET_SIZE, "payload too large");
        let addr = self // wtf rust
            .registry
            .lock()
            .unwrap()
            .clients
            .get(&id)
            .map(|c| c.addr)
            .ok_or_else(|| anyhow::anyhow!("unknown client {id}"))?;
        self.socket.try_send_to(&protocol::encode(PacketType::DATA, data), addr)?;
        return Ok(());
    }

    pub fn broadcast(&self, data: &[u8]) {
        if data.len() > MAX_PACKET_SIZE {
            return;
        }

        let packet = protocol::encode(PacketType::DATA, data);
        let addrs: Vec<SocketAddr> = self
            .registry
            .lock()
            .unwrap()
            .clients
            .values()
            .map(|c| c.addr)
            .collect();

        for a in addrs {
            let _ = self.socket.try_send_to(&packet, a);
        }
    }

    pub fn kick(&self, id: ClientId) {
        if let Some(c) = self.registry.lock().unwrap().remove(id) {
            let _ = self.socket.try_send_to(
                &protocol::encode(PacketType::DISCONNECT, &[]), c.addr
            );
        }
    }

    // ----- Getters -----
    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        return self.socket.local_addr();
    }

    pub fn poll(&self) -> Vec<RawPacket> {
        return std::mem::take(&mut *self.inbox.lock().unwrap());
    }

    pub fn client_list(&self) -> Vec<ClientInfo> {
        return self.registry.lock().unwrap().clients.values().cloned().collect();
    }

    pub fn client_count(&self) -> usize {
        return self.registry.lock().unwrap().clients.len();
    }


    // ----- Background Tasks -----

    /// background task for recieving packets
    async fn recv_loop(
        socket: Arc<UdpSocket>,
        inbox: Arc<Mutex<Vec<RawPacket>>>,
        registry: Arc<Mutex<Registry>>,
    ) {
        let mut buffer = vec![0u8; MAX_PACKET_SIZE + 1];
        loop {
            // checks for incoming packets
            let (psize, addr) = match socket.recv_from(&mut buffer).await {
                Ok(v) => v,
                Err(_) => {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    continue;
                }
            };

            if psize > MAX_PACKET_SIZE {
                continue;
            }
            let Some((kind, body)) = protocol::parse(&buffer[..psize]) else {
                continue;
            };

            match kind { // todo change to work for encrypted packets
                PacketType::CONNECT => {
                    let id = registry.lock().unwrap().register(addr);
                    if let Some(id) = id {
                        let reply = protocol::encode(
                            PacketType::ACCEPT, 
                            &id.to_le_bytes(),
                        );

                        let _ = socket.send_to(&reply, addr).await;
                    }
                },
                PacketType::ACCEPT => {},
                PacketType::DATA => {
                    let id = registry.lock().unwrap().touch_addr(&addr);
                    if let Some(id) = id {
                        let mut q = inbox.lock().unwrap();
                        if q.len() < MAX_INBOX {
                            q.push(RawPacket {
                                client_id: id,
                                data: body.to_vec(),
                            });
                        }
                    }
                },
                PacketType::DISCONNECT => {
                    let mut registry = registry.lock().unwrap();
                    if let Some(&id) = registry.addr_to_client.get(&addr) {
                        registry.remove(id);
                    }
                },
                PacketType::KEEP_ALIVE => {
                    registry.lock().unwrap().touch_addr(&addr);
                },
            }
        }
    }

    /// kick players that are inactive
    async fn timeout_loop(
        socket: Arc<UdpSocket>, 
        registry: Arc<Mutex<Registry>>,
    ) {
        let mut tick = tokio::time::interval(Duration::from_secs(1));
        loop {
            tick.tick().await;
            let dead_conn: Vec<ClientInfo> = {
                let mut r = registry.lock().unwrap();
                let ids: Vec<ClientId> = r
                    .clients
                    .values()
                    .filter(|c| c.last_packet.elapsed() > CLIENT_TIMEOUT)
                    .map(|c| c.id)
                    .collect();
                ids.into_iter().filter_map(|id| r.remove(id)).collect()
            };

            for conn in dead_conn {
                let _ = socket.try_send_to(
                    &protocol::encode(PacketType::DISCONNECT, &[]), 
                    conn.addr,
                );
            }
        }
    }
}


impl Drop for NetworkServer {
    fn drop(&mut self) {
        let packet = protocol::encode(PacketType::DISCONNECT, &[]);
        for client in self.registry.lock().unwrap().clients.values() {
            let _ = self.socket.try_send_to(&packet, client.addr);
        }
        for t in &self.tasks {
            t.abort();
        }
    }
}
