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

pub mod client_data;

use crate::engine::net::packet::{IncomingPacket, OutgoingPacket};
use crate::engine::net::server::client_data;


pub struct ServerDesc {
    pub port: u64,
    pub max_clients: u8, // should max 256, more is absurd
}

pub struct Server {
    private_key: String, // unecrypts all incoming packets
    public_key: String, // sent to all connected clients

    // connected: [Client; 8], // all connected clients + open slots

    clients: Arc<RwLock<HashMap<ClientData>>>,

    incoming: Receiver<IncomingPacket>,
    outgoing: Sender<OutgoingPacket>,
}

impl Server {
    pub async fn create(
        server_desc: ServerDesc,
        server_callback: fn(),
    ) -> io::Result<Self> {
        let listener = UdpSocket::bind(format!("localhost:{}", server_desc.port)).await?;
        let listener = Arc::new(listener);



        return Ok(Self {
            private_key: todo!(),
            public_key: todo!(),
        });
    }

    pub async fn start(&mut self) {

    }

    pub async fn stop(&mut self) {

    }

    pub async fn send_to() {

    }

}

// struct ConnectionSlot {
//     latest_tick: u64,
//     latest_playload: Vec<u8>,
// }
//
// pub struct Server {
//     max_connections: Arc<u32>,
//     socket: Arc<UdpSocket>,
//     connections: Arc<Mutex<HashMap<SocketAddr, ConnectionSlot>>>,
// }
//
// impl Server {
//     pub async fn create(
//         port: u32,
//         max_connections: u32,
//     ) -> anyhow::Result<Self> {
//         let socket = UdpSocket::bind(format!("localhost:{}", port)).await?;
//         let socket = Arc::new(socket);
//         let connections = Arc::new(Mutex::new(HashMap::new()));
//         let connections_cap = Arc::new(max_connections);
//
//         // server thread created
//         let server_max_connections = Arc::clone(&connections_cap);
//         let server_socket = Arc::clone(&socket);
//         let server_connections = Arc::clone(&connections);
//
//         std::thread::spawn(move || Self::server_thread(
//             server_max_connections, 
//             server_socket, 
//             server_connections,
//         ));
//
//         return Ok(Self {
//             max_connections: connections_cap,
//             socket, 
//             connections,
//         });
//     }
//
//     async fn send_all(&self, data: &[u8]) {
//         let conns = self.connections.lock().unwrap();
//         for addr in conns.keys() {
//             let _ = self.send_to(addr, data).await;
//         }
//     }
//     async fn send_to(
//         &self,
//         addr: &SocketAddr,
//         data: &[u8],
//     ) -> std::io::Result<usize> {
//         return self.socket.send_to(data, addr).await;
//     }
//
//     // Server handling thread:
//     // - connections
//     // - all authoritative server logic
//     // - 
//     fn server_thread(
//         _max_connections: Arc<u32>,
//         _recv_socket: Arc<UdpSocket>, 
//         _connections: Arc<Mutex<HashMap<SocketAddr, ConnectionSlot>>>,
//     ) {
//         let mut buffer: [u8; 1500]; // max ethernet packet size, if get split, rip
//
//         loop {
//         }
//     }
// }
