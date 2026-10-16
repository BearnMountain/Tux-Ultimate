use std::{sync::Arc};
use winit::{window::{Window}};

pub mod single_player;
pub mod multiplayer;
pub mod server;
pub mod ui;
pub mod game_session;
pub mod simulation;
pub mod spector;

use crate::{
    engine::Engine, game::{game_session::GameSession, multiplayer::MultiplayerSession, simulation::game_state::GameState, single_player::SinglePlayerSession, spector::SpectorSession, ui::command::UiCommand}, 
};

#[allow(non_camel_case_types)]
pub enum ActiveGameSession {
    /// game simulation hosted locally
    SINGLE_PLAYER(SinglePlayerSession),
    /// connect to server you host or remotely
    MULTIPLAYER(MultiplayerSession),
    /// preview hosting server
    SPECTOR(SpectorSession),
}

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
    session: Option<ActiveGameSession>,

    // general stuff
    pub engine: Engine,
    // pub input_handler: input::GameInput,
}

impl Game {
    pub fn init(
        window: Arc<Window>,
    ) -> Self {
        // creates everything needed to run a game


        // graphics and ui created
        let engine = Engine::new(window.clone());
        // let input_handler = input::GameInput::new();

        return Self {
            ui: ui::UI::init(
                &engine.renderer.get_ui()
            ),
            tick: 0,
            session: None,
            engine,
            // input_handler,
        };
    }

    pub fn init_single_player_session(&mut self) {
        self.shutdown_session();
        self.session = Some(ActiveGameSession::SINGLE_PLAYER(
            SinglePlayerSession::init()
        ));
    }
    pub fn init_multiplayer_session(&mut self) {
        self.shutdown_session();
        self.session = Some(ActiveGameSession::MULTIPLAYER(
            MultiplayerSession::init()
        ));
    }
    pub fn init_spector_session(&mut self) {
        self.shutdown_session();
        self.session = Some(ActiveGameSession::SPECTOR(
            SpectorSession::init()
        ));
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

        if let Some(session) = &mut self.session {
            match session {
                ActiveGameSession::SINGLE_PLAYER(single_player_session) =>
                    single_player_session.frame(),
                ActiveGameSession::MULTIPLAYER(multiplayer_session) =>
                    multiplayer_session.frame(),
                ActiveGameSession::SPECTOR(spector_session) =>
                    spector_session.frame(),
            }
        }

        self.engine.renderer.render()?;

        return Ok(());
    }

    /// called every tick(game updates, server, etc)
    pub fn tick(&mut self) {
        self.tick += 1;

        if let Some(session) = &mut self.session {
            match session {
                ActiveGameSession::SINGLE_PLAYER(single_player_session) =>
                    single_player_session.tick(),
                ActiveGameSession::MULTIPLAYER(multiplayer_session) =>
                    multiplayer_session.tick(),
                ActiveGameSession::SPECTOR(spector_session) =>
                    spector_session.tick(),
            }
        }
    }

    fn shutdown_session(&mut self) {
        if let Some(session) = &mut self.session {
            match session {
                ActiveGameSession::SINGLE_PLAYER(single_player_session) => 
                    single_player_session.shutdown(),
                ActiveGameSession::MULTIPLAYER(multiplayer_session) =>
                    multiplayer_session.shutdown(),
                ActiveGameSession::SPECTOR(spector_session) =>
                    spector_session.shutdown(),
            }
        }

        self.session = None;
    }
}
