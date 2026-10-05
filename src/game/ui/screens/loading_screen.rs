use crate::game::ui::{command::UiCommand, screens::Screen};



pub struct LoadingScreen {

}

impl Screen for LoadingScreen {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<UiCommand> {
        println!("map selection menu not impl");
        return vec![UiCommand::None];
    }
}

impl LoadingScreen {
    pub fn new() -> Self {
        return Self {

        };
    }
}
