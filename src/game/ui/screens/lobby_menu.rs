#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LobbyMenuAction {
    NONE,
}

// #[allow(non_camel_case_types)]
// #[derive(Clone, Copy, PartialEq, Eq)]
// enum LobbyTab {
//     SERVER_BROWSER,
//     DIRECT_CONNECT,
//     HOST_SERVER,
// }

pub struct LobbyMenu {

}

impl LobbyMenu {
    pub fn new() -> Self {
        return Self {

        };
    }

    pub fn ui(&mut self, ui: &egui::Ui) {
    }
}
