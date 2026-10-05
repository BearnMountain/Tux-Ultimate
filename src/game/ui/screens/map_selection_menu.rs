use crate::game::ui::{command::UiCommand, screens::Screen};



pub struct MapSelectionMenu {

}

impl Screen for MapSelectionMenu {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<UiCommand> {
        println!("map selection menu not impl");
        return vec![UiCommand::None];
    }
}

impl MapSelectionMenu {
    pub fn new() -> Self {
        return Self {

        };
    }
}
