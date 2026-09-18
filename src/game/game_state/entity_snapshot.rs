use std::net::SocketAddr;

use glam::Vec2;

use crate::engine::net::server::ClientId;

pub struct EntitySnapshot {
    id: ClientId,
    position: Vec2,
    velocity: Vec2,
    rotation: f32,
    health: u16,
}
