use crate::engine::net::packet::MAX_PACKET_SIZE;

/// type of packet being sent, helps (de)serialize packets
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PacketType {
    CONNECT = 0x0, // client -> server
    ACCEPT = 0x1, // server -> client after handshake
    DATA = 0x2, // both ways: to client = game_state, to server = inputs
    DISCONNECT = 0x3, // both ways
    KEEP_ALIVE = 0x4, // client -> server
}

impl PacketType {
    pub fn from_u8(c: u8) -> Option<Self> {
        Some(match c {
            0x0 => Self::CONNECT,
            0x1 => Self::ACCEPT,
            0x2 => Self::DATA,
            0x3 => Self::DISCONNECT,
            0x4 => Self::KEEP_ALIVE,
            _ => return None,
        })
    }
}

/// return (type + raw data)
pub fn parse(buf: &[u8]) -> Option<(PacketType, &[u8])> {
    let (&k, body) = buf.split_first()?;
    return Some((PacketType::from_u8(k)?, body));
}

/// caller ensure len < MAX_PACKET_SIZE
pub fn encode(kind: PacketType, body: &[u8]) -> Vec<u8> {
    debug_assert!(body.len() < MAX_PACKET_SIZE);
    let mut v = Vec::with_capacity(1 + body.len());
    v.push(kind as u8);
    v.extend_from_slice(body);
    return v;
}
