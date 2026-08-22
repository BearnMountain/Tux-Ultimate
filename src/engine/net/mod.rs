pub mod server;
pub mod client;
pub mod connection;

use tokio::net::UdpSocket;

use crate::engine::net::{
    server::Server, 
    client::Client
};

/*
user creates server: 
- server is threaded process
- creates db that queries incoming packets(deserializes them)
    - stores in sorted by tick data struct
- queries packets closest to current server tick
- Updates Game State:
    - runs everything through physics first
    - collision remediation
    - ...
- broadcasts all packets

user creates client:
- creates thread query queue for server packets + deserialization
- main process '.get_packet()' for state
- client broadcasts all current keyinputs back to server
*/

// pub enum NetworkOptions {
//
// }

pub struct Network {
    pub client: Option<Client>,
    pub server: Option<Server>,
}

impl Network {

}
