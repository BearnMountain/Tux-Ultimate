use std::{net::SocketAddr, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ServerAddress(pub SocketAddr);

impl FromStr for ServerAddress {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

pub mod server;
pub mod client;
pub mod protocol;
pub mod packet;
