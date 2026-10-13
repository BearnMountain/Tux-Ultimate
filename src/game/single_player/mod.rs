use crate::game::game_session::GameSession;

pub struct SinglePlayerSession {
    tick: u64,
}

impl GameSession for SinglePlayerSession {
    fn tick(&mut self, dt: f32) {
        todo!()
    }

    fn update(&mut self, dt: f32) {
        todo!()
    }

    fn shutdown(&mut self) {
        todo!()
    }
}
