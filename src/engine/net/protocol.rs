

/// type of packet being sent, helps (de)serialize packets
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PacketType {
    HANDSHAKE = 0x1,
    CLIENT_INPUT = 0x2,
    SERVER_STATE = 0x3,
    CHAT = 0x4,
}

impl PacketType {
    pub fn from_u8(c: u8) -> Option<Self> {
        match c {
            0x1 => Some(PacketType::HANDSHAKE),
            0x2 => Some(PacketType::CLIENT_INPUT),
            0x3 => Some(PacketType::SERVER_STATE),
            0x4 => Some(PacketType::CHAT),
            _ => None,
        }
    }
}

/// client pre-connection
/// - carries ephemeral pub keys
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum HandshakeMessage {
    CLIENT_HELLO { public_key: [u8; 32] },
    SERVER_HELLO { public_key: [u8; 32] },
}

/// A frame of inputs sent from client to server
#[derive(Debug, Clone)]
pub struct InputFrame {
    pub sequence: u32,
    pub inputs: Vec<u16>,
}
