use crate::game::ui::{command::UiCommand, screens::Screen};



pub struct SinglePlayer {

}

impl Screen for SinglePlayer {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<UiCommand> {
        println!("single player not impl");
        return vec![UiCommand::None];
    }
}

impl SinglePlayer {
    pub fn new() -> Self {
        return Self {

        };
    }
}
