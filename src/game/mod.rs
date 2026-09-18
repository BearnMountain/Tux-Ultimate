use std::{sync::Arc};
use winit::{window::{Window}};

pub mod client;
pub mod server;
pub mod ui;
pub mod input;

use crate::{engine::Engine, game::{client::GameClient, ui::command::UiCommand}};

// max time that a frame isnt updated

/// Game is the gui interface that encapsulates all
/// rendered items + user inputs
///
/// Notes:
/// - Client: game client is for rendering video game state and such
/// - Server: game server is for the server interface, doesnt need
///   heavy wgpu rendering, so under wgpu is live loaded from menu
pub struct Game {
    tick: u64,

    // game specific
    ui: ui::UI,
    game_client: GameClient,

    // general stuff
    pub engine: Engine,
    pub input_handler: input::GameInput,
}

impl Game {
    pub fn init(
        window: Arc<Window>,
    ) -> Self {
        // creates everything needed to run a game
        // graphics and ui created
        let engine = Engine::new(window.clone());
        let input_handler = input::GameInput::new();

        return Self {
            ui: ui::UI::init(
                &engine.renderer.get_ui()
            ),
            tick: 0,
            game_client: GameClient::init(window),
            engine,
            input_handler,
        };
    }

    /// called at monitor refresh rate(just for graphics)
    pub fn frame(&mut self) -> anyhow::Result<()> {
        self.engine.begin_ui();

        let mut commands: Vec<UiCommand> = Vec::new();
        self.engine.ui(|ui| {
            #[cfg(debug_assertions)] {
                ui.ctx().set_debug_on_hover(true);
            }
            commands = self.ui.frame(ui);
        });

        // handle commands
        if !commands.is_empty() {
            println!("{:#?}", commands);
        }


        self.engine.end_ui();

        self.engine.renderer.render()?;

        return Ok(());
    }

    /// called every tick(game updates, server, etc)
    pub fn update(&mut self) {
        self.tick += 1;
    }

}
