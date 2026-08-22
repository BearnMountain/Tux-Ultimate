use std::net::UdpSocket;

use tokio::io::BufWriter;



pub struct Connection {
    stream: BufWriter<UdpSocket>,
    buffer: Vec<u8>,
}

impl Connection {
    pub fn new(socket: UdpSocket) -> Connection {
        return Connection {
            stream: BufWriter::new(socket),
            buffer: Vec::with_capacity(4 * 1024),
        };
    }
}
