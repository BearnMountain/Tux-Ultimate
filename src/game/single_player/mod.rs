use crate::game::{game_session::GameSession, simulation::game_state::GameState};

pub struct SinglePlayerSession {
    tick: u64,
}

impl GameSession for SinglePlayerSession {
    fn tick(&mut self) {
        todo!()
    }

    fn frame(&mut self) {
        todo!()
    }

    fn shutdown(&mut self) {
        todo!()
    }
}

impl SinglePlayerSession {
    pub fn init() -> Self {


        return Self {
            tick: 0,
        };
    }
}
