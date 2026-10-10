use std::net::SocketAddr;

pub type ClientId = u8;

pub const MAX_PACKET_SIZE: usize = 1200; // just short of max MTU size

pub struct RawPacket {
    pub client: Option<ClientId>,
    pub data: [u8; MAX_PACKET_SIZE],
    pub addr: SocketAddr,
}

// type ClientId = u64;
//
// pub struct IncomingPacket {
//     pub client_id: ClientId,
//     pub packet: Vec<u8>,
// }
//
// pub struct OutgoingPacket {
//     pub destination: Destination,
//     pub packet: Vec<u8>,
// }
//
//
// pub struct Packet {
//
// }
//
// impl Packet {
//     // pub fn serialize() -> 
// }
