#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameMode {
    FREE_FOR_ALL,
    DEATH_MATCH,    
    TEAM_DEATH_MATCH,
    RANDOM,
}

impl GameMode {
    pub fn get(i: usize) -> GameMode {
        match i {
            0 => GameMode::FREE_FOR_ALL,
            1 => GameMode::DEATH_MATCH,
            2 => GameMode::TEAM_DEATH_MATCH,
            _ => GameMode::RANDOM,
        }
    }

    pub fn to_list() -> &'static [&'static str] {
        return &[
            "Free for All",
            "Deathmatch",
            "Team Deathmatch",
            "Random",
        ];
    }
}
