use std::{sync::{Arc, mpsc}, thread::JoinHandle, time::{Duration, Instant}};

use tokio::runtime::Runtime;

use crate::{engine::net::{protocol::InputFrame, server::NetworkServer}, game::server::simulation::GameSimulation};

pub mod net;
pub mod game_mode;
pub mod simulation;

// MTU is 1500 bytes
#[derive(Clone)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub tick_rate: u16, // Hz
    pub max_players: u8,
}

impl Default for ServerConfig {
    fn default() -> Self {
        return Self { 
            bind_addr: "0.0.0.0".to_string(), 
            tick_rate: 60, 
            max_players: 8,
        };
    }
}

/// Game Server runs on its own thread
///
/// Usage: 
/// ```
/// let server = GameServer::new(ServerConfig::default());
/// server.spawn(); // and thats it, runs in background
/// server.shutdown(); // at end
/// ```
#[derive(Clone)]
pub struct GameServer {
    config: ServerConfig,
    shutdown_tx: Option<mpsc::Sender<()>>, // transmit shutdown req
}

impl GameServer {
    pub fn new(config: ServerConfig) -> Self {
        return Self {
            config,
            shutdown_tx: None,
        };
    }

    /// thread in background
    pub fn spawn(&mut self) -> io::Result<JoinHandle<()>> {
        let runtime = Runtime::new()?;

        let (shutdown_tx, shutdown_rx) = mpsc::channel();
        let config = self.config.clone();

        // spawn thread
        let handle = std::thread::Builder::new()
            .name("game-server".into())
            .spawn(move || {
                runtime.block_on(async move {
                    if let Err(e) = run_server(config, shutdown_rx).await {
                        log::error!("[game-server] fatal error: {}", e);
                    }
                });
            })?; // fails if os cant create thread

        self.shutdown_tx = Some(shutdown_tx);
        log::info!("[game-server] spawned on background thread");
        return Ok(handle);
    }

    pub fn shutdown(&self) {
        if let Some(tx) = &self.shutdown_tx {
            let _ = tx.try_send(());
        }
    }
}

/// main server loop
async fn run_server(
    config: ServerConfig,
    mut shutdown_rx: mpsc::Receiver<()>,
) -> anyhow::Result<()> {
    // init net layer
    let network = Arc::new(NetworkServer::bind(&config.bind_addr).await?);
    let mut game_world = GameSimulation::init();

    // channel to obtain input frames from the network layer
    let (input_tx, mut input_rx) = mpsc::channel::<(u64, InputFrame)>();

    // handles handshake + input packets
    let network_clone = Arc::clone(&network);
    tokio::spawn(async move {
        network_clone.run().await;
    });

    // game loop
    let tick_duration = Duration::from_millis(1000 / config.tick_rate as u64);
    let mut last_tick = Instant::now();
    let mut tick: u64 = 0;

    log::info!("[game-server] game loop starting at {} Hz", config.tick_rate);

    loop {
        if shutdown_rx.try_recv().is_ok() {
            log::info!("[game-server] shutting down");
            break;
        }

        // wait for tick
        let elapsed = last_tick.elapsed();
        if elapsed < tick_duration {
            tokio::time::sleep(tick_duration - elapsed).await;
        }
        last_tick = Instant::now();
        tick += 1;

        // 1. process inputs
        while let Ok((client_id, frame)) = input_rx.try_recv() {
            // process inputs: game_world, client_id, frame
        }

        // 2. physics sim
        // 3. update game state
        // 4. broadcast state to all clients
        // 5. cleanup
    }

    return Ok(());
}

