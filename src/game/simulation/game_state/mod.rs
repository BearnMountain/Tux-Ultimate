pub mod entity_snapshot;

pub struct GameState {
    server_tick: u64,

    entities: Vec<entity_snapshot::EntitySnapshot>,
}

impl GameState {

    pub fn test() -> Self {
        return Self {
            server_tick: 0,
            entities: Vec::new(),
        };
    }
}
