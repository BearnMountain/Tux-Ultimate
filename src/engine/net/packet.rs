use crate::engine::net::protocol::Destination;

type ClientId = u64;

pub struct IncomingPacket {
    pub client_id: ClientId,
    pub packet: Vec<u8>,
}

pub struct OutgoingPacket {
    pub destination: Destination,
    pub packet: Vec<u8>,
}


pub struct Packet {

}

impl Packet {
    // pub fn serialize() -> 
}
