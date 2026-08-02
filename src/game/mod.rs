use winit::event::WindowEvent;

mod io;

pub struct Game {
    pub input_handler: io::input::Input,
}

impl Game {
    pub fn new() -> Self {
        return Self {
            input_handler: io::input::Input::new(),
        };
    }

    pub fn run(&mut self) {

    }

}
