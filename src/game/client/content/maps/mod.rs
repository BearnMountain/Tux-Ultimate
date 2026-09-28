

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameMaps {
    BONGO_BAY,
    RANDOM,
}

impl GameMaps {
    pub fn to_list(&mut self) -> Vec<String> {
        return vec![
            "Bongo Bay".into(),
            "Random".into(),
        ];
    }
}
