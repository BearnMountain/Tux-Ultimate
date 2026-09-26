use crate::game::{client::content::maps::GameMaps, server::game_mode::GameMode};



#[derive(Debug)]
pub struct GameServerConfig {
    pub server_name: Option<String>, // defaults to ip
    pub password: Option<String>,
    pub port: u32,

    // ----- Match Specifics -----
    pub max_players: u8,
    pub game_mode: GameMode,
    pub map: GameMaps,
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
            server_name: Some("MyGame".into()), // defaults to ip
            password: None,
            port: 2048,
            max_players: 8,
            game_mode: GameMode::FREE_FOR_ALL,
            map: GameMaps::NONE,
            tick_rate: 60,
        };
    }
}


