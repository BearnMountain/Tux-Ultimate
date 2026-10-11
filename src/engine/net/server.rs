use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tokio::net::UdpSocket;
use tokio::sync::RwLock;
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
    // return new slot or existing id if already registered
    // fn register(&mut self, addr: SocketAddr) -> Option<ClientId> {
    //     if let Some(&id) = self.addr_to_client.get(&addr) {
    //     }
    // }
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
        });
    }

    /// server packet reciever in the background
    pub async fn run(&self) {

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
                PacketType::CONNECT => {}
                PacketType::ACCEPT => {},
                PacketType::DATA => {},
                PacketType::DISCONNECT => {},
                PacketType::KEEP_ALIVE => {},
            }
        }
    }

    /// kick players that are inactive
    async fn timeout_loop(
        socket: Arc<UdpSocket>, 
        registry: Arc<Mutex<Registry>>,
    ) {

    }

}


// mod client_data;
//
// use std::{collections::HashMap, net::{SocketAddr, UdpSocket}, sync::Arc};
//
// use tokio::sync::{RwLock, mpsc};
//
// use crate::engine::net::packet::{IncomingPacket, OutgoingPacket};
//
// pub enum PacketTarget {
//     Client(ClientId),
//     Broadcast,
// }
//
// pub struct ServerPacket {
//     pub data: Vec<u8>,
// }
//
// pub struct ServerClient {
//     pub addr: SocketAddr,
// }
//
// pub struct Server {
//     // private_key: String, // unecrypts all incoming packets
//     // public_key: String, // sent to all connected clients
//
//     // connected: [Client; 8], // all connected clients + open slots
//
//     socket: Arc<UdpSocket>,
//
//     incoming: mpsc::Receiver<(SocketAddr, Packet)>,
//     outgoing: mpsc::Sender<(SocketAddr, Packet)>,
//
//     clients: Arc<RwLock<HashMap<SocketAddr, Client>>>,
// }
//
// impl Server {
//     pub async fn create(
//         addr: &str
//     ) -> io::Result<Self> {
//         let socket = Arc::new(UdpSocket::bind(addr).await?);
//         let (incoming_sender, incoming_reciever) = mpsc::channel(1024);
//         let (outgoing_sender, mut outgoing_reciever) = mpsc::channel(1024);
//
//         let clients = Arc::new(RwLock::new(HashMap::new()));
//
//         // recieving packets
//         {
//             let socket = Arc::clone(&socket);
//             let incoming_sender = incoming_sender.clone();
//             let clients = Arc::clone(&clients);
//
//             tokio::spawn(async move {
//                 let mut buffer = [0u8; 65535];
//
//                 loop {
//                     let result = socket.recv_from(&mut buffer).await;
//
//                     let (len, addr) = match result {
//                         Ok(result) => result,
//                         Err(e) => {
//                             eprintln!("UDP receive error: {e}");
//                             break;
//                         }
//                     };
//
//                     // registers new client
//                     {
//                         let mut clients = clients.write().await;
//                         clients.entry(addr).or_insert(|| Client {
//                             addr,
//                         });
//                     }
//
//                     // server shutdown
//                     if incoming_sender.send((addr, packet).await.is_err()) {
//                         break; 
//                     }
//
//
//                 }
//             });
//         }
//
//         // sending packets
//         {
//             let socket = Arc::clone(&socket);
//
//             tokio::spawn(async move {
//                 while let Some((addr, packet)) = outgoing_reciever.recv().await {
//                     if let Err(e) = socket.send_to(&packet.data, addr).await {
//                         eprintln!("UDP send error to {addr}: {e}");
//                     }
//                 }
//             });
//         }
//
//         return Ok(Self {
//             socket,
//             incoming: incoming_reciever,
//             outgoing: outgoing_sender,
//             clients,
//         });
//     }
//
//     pub fn try_recv(
//         &mut self,
//     ) -> Option<(SocketAddr, Packet)> {
//         return self.incoming.try_recv().ok();
//     }
//
//     pub async fn send_to(&self, addr: SocketAddr, packet: Packet) {
//
//     }
//
//     pub fn broadcast(&self, packet: Packet) {
//
//     }
//
//     pub async fn clients(&self) -> Vec<SocketAddr> {
//         self.clients.read().await.keys().copied().collect(); 
//     }
//
// }
