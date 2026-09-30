

#[derive(Debug)]
pub struct GameRules {
    pub lives: u8,
    pub rounds: u8,
    pub round_length_sec: u16,
    pub sudden_death_sec: u16,
    pub damage_scalar: f32,
    pub health_scalar: f32,
}

impl Default for GameRules {
    fn default() -> Self {
        return Self {
            lives: 3,
            rounds: 1,
            round_length_sec: 180,
            sudden_death_sec: 60,
            damage_scalar: 1.0,
            health_scalar: 1.0,
        };
    }
}
