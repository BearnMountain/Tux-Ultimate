use crate::{engine::net::ServerAddress, game::{server::net::config::GameServerConfig, ui::screens::ScreenId}};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum UiCommand {
    // Navigation
    Push(ScreenId), 
    Pop, // go back button
    PopTo(ScreenId), // pop to n(usually to main menu root)

    // Server Stuff
    ConnectToServer(ServerAddress),
    HostServer(GameServerConfig),
    // CancelConnection,
    // SelectCharacter/Map
    // StartMatch

    // System
    Quit,
    // UpdateSettings(SettingsConfig),

    None,
}
