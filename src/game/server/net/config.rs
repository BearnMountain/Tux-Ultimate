use crate::game::{client::content::maps::GameMaps, server::{game_mode::GameMode, net::game_rules::GameRules}};



#[derive(Debug)]
pub struct GameServerConfig {
    pub server_name: String, // defaults to ip
    pub password: Option<String>,
    pub port: u32,

    // ----- Match Specifics -----
    pub max_players: u8,
    pub game_mode: GameMode,
    pub game_map: GameMaps,
    pub game_rules: GameRules,
    pub tick_rate: u16, // in hertz



    /* Extension:
        - Round time
        - Lives
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
            server_name: "MyGame".into(), // defaults to ip
            password: None,
            port: 2048,
            max_players: 8,
            game_mode: GameMode::FREE_FOR_ALL,
            game_map: GameMaps::RANDOM,
            game_rules: GameRules::default(),
            tick_rate: 60,
        };
    }
}


