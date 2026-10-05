use crate::game::ui::{command::UiCommand, screens::Screen};



pub struct ExtrasScreen {

}

impl Screen for ExtrasScreen {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<UiCommand> {
        println!("map selection menu not impl");
        return vec![UiCommand::None];
    }
}

impl ExtrasScreen {
    pub fn new() -> Self {
        return Self {

        };
    }
}
