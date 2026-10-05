

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameMaps {
    BONGO_BAY,
    RANDOM,
}

impl GameMaps {
    pub const TOTAL_MAPS: usize = GameMaps::RANDOM as usize + 1;

    pub fn get(i: usize) -> GameMaps {
        match i {
            0 => GameMaps::BONGO_BAY,
            _ => GameMaps::RANDOM,
        }
    }

    pub fn to_list(&mut self) -> &'static[&'static str] {
        return &[
            "Bongo Bay",
            "Random",
        ];
    }
}
