#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameCharacters {
    TUX,
    TEST,
    RANDOM,
}

impl GameCharacters {
    pub const TOTAL_CHARACTERS: usize = GameCharacters::RANDOM as usize + 1;

    pub fn get(i: usize) -> GameCharacters {
        match i {
            0 => GameCharacters::TUX,
            _ => GameCharacters::RANDOM,
        }
    }

    pub fn to_list() -> &'static [&'static str] {
        return &[
            "Tux",
            "Random",
        ];
    }
}

pub trait CharacterInterface {

}
