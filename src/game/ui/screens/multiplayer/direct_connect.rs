use crate::game::ui::{command::UiCommand, screens::Screen};

pub struct DirectConnect {
    
}

impl Screen for DirectConnect {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<UiCommand> {
        println!("direct_connect not impl yet");  
        return vec![UiCommand::None];
    }
}

impl DirectConnect {
    pub fn new() -> Self {


        return Self {

        };
    }
}
