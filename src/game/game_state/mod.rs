pub mod entity_snapshot;

pub struct GameState {
    server_tick: u64,

    entities: Vec<entity_snapshot::EntitySnapshot>,
}
