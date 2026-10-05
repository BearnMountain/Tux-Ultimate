use crate::game::{client::content::{characters::GameCharacters, maps::GameMaps::{self, BONGO_BAY}}, server::{game_mode::GameMode, net::game_rules::GameRules}};



#[derive(Debug, PartialEq, Clone)]
pub struct GameServerConfig {
    // ----- Match Specifics -----
    // server options
    pub server_name: String, // defaults to ip
    pub password: Option<String>,
    pub port: u32,
    pub max_players: u8,
    pub tick_rate: u16, // in hertz

    // game content
    pub game_map: Vec<(GameMaps, bool)>,
    pub game_character: Vec<(GameCharacters, bool)>,

    // game options
    pub game_mode: GameMode,
    pub game_rules: GameRules,

    /* Extension:
        - Respawn time
        - Multipliers: health, damage, etc
        - Teams enabled
            - team size
            - etc
        - Bots
            - Bot count
        - Gravity
        - Player movement settings
        - Item/weapon/ability restrictions
    */
}

impl Default for GameServerConfig {
    fn default() -> Self {
        return Self {
            server_name: "localhost".into(),
            password: None,
            port: 2048,
            max_players: 8,
            tick_rate: 60,
            game_map: vec![
                (GameMaps::BONGO_BAY, true),
                (GameMaps::RANDOM, true),
            ],
            game_character: vec![
                (GameCharacters::TUX, true),
                (GameCharacters::RANDOM, true),
            ],
            game_mode: GameMode::RANDOM,
            game_rules: GameRules::default(),
        };
    }
}


