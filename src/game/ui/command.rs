use crate::game::ui::screens::ScreenId;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UiCommand {
    // Navigation
    Push(ScreenId), 
    Pop, // go back button
    PopTo(ScreenId), // pop to n(usually to main menu root)

    // Online
    // ConnectToServer(ServerAddress),
    // CancelConnection,
    // HostServer(GameServerConfig),
    // SelectCharacter/Map
    // StartMatch

    // System
    Quit,
    // UpdateSettings(SettingsConfig),

    None,
}
