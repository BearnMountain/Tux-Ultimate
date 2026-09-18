/*
Server paradigm:
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

mod client_data;

use std::{collections::HashMap, net::{SocketAddr, UdpSocket}, sync::Arc};

use tokio::sync::{RwLock, mpsc};

use crate::engine::net::packet::{IncomingPacket, OutgoingPacket};

pub enum PacketTarget {
    Client(ClientId),
    Broadcast,
}

pub struct ServerPacket {
    pub data: Vec<u8>,
}

pub struct ServerClient {
    pub addr: SocketAddr,
}

pub struct Server {
    // private_key: String, // unecrypts all incoming packets
    // public_key: String, // sent to all connected clients

    // connected: [Client; 8], // all connected clients + open slots

    socket: Arc<UdpSocket>,

    incoming: mpsc::Receiver<(SocketAddr, Packet)>,
    outgoing: mpsc::Sender<(SocketAddr, Packet)>,

    clients: Arc<RwLock<HashMap<SocketAddr, Client>>>,
}

impl Server {
    pub async fn create(
        addr: &str
    ) -> io::Result<Self> {
        let socket = Arc::new(UdpSocket::bind(addr).await?);
        let (incoming_sender, incoming_reciever) = mpsc::channel(1024);
        let (outgoing_sender, mut outgoing_reciever) = mpsc::channel(1024);

        let clients = Arc::new(RwLock::new(HashMap::new()));
        
        // recieving packets
        {
            let socket = Arc::clone(&socket);
            let incoming_sender = incoming_sender.clone();
            let clients = Arc::clone(&clients);

            tokio::spawn(async move {
                let mut buffer = [0u8; 65535];

                loop {
                    let result = socket.recv_from(&mut buffer).await;

                    let (len, addr) = match result {
                        Ok(result) => result,
                        Err(e) => {
                            eprintln!("UDP receive error: {e}");
                            break;
                        }
                    };

                    // registers new client
                    {
                        let mut clients = clients.write().await;
                        clients.entry(addr).or_insert(|| Client {
                            addr,
                        });
                    }

                    // server shutdown
                    if incoming_sender.send((addr, packet).await.is_err()) {
                        break; 
                    }


                }
            });
        }

        // sending packets
        {
            let socket = Arc::clone(&socket);

            tokio::spawn(async move {
                while let Some((addr, packet)) = outgoing_reciever.recv().await {
                    if let Err(e) = socket.send_to(&packet.data, addr).await {
                        eprintln!("UDP send error to {addr}: {e}");
                    }
                }
            });
        }

        return Ok(Self {
            socket,
            incoming: incoming_reciever,
            outgoing: outgoing_sender,
            clients,
        });
    }

    pub fn try_recv(
        &mut self,
    ) -> Option<(SocketAddr, Packet)> {
        return self.incoming.try_recv().ok();
    }

    pub async fn send_to(&self, addr: SocketAddr, packet: Packet) {

    }

    pub fn broadcast(&self, packet: Packet) {

    }

    pub async fn clients(&self) -> Vec<SocketAddr> {
        self.clients.read().await.keys().copied().collect(); 
    }
    
}
