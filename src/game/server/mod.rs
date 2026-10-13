use std::{io, sync::Arc, thread::JoinHandle, time::Duration};
use tokio::{
    runtime::Builder,
    sync::{mpsc, oneshot},
    time::{interval, MissedTickBehavior},
};

use crate::{engine::net::server::NetworkServer, game::server::{config::GameServerConfig, simulation::GameSimulation}};

pub mod simulation;
pub mod config;
pub mod game_mode;
pub mod game_rules;

/// Game server running on its own thread.
///
/// ```ignore
/// let mut server = GameServer::new(ServerConfig::default());
/// server.startup()?;
/// // ...
/// server.shutdown(); // also called on drop
/// ```
pub struct GameServer {
    config: GameServerConfig,
    shutdown_tx: Option<oneshot::Sender<()>>, // tunnel to shutdown thread
    handle: Option<JoinHandle<()>>, 
}

impl GameServer {
    pub fn new(config: GameServerConfig) -> Self {
        Self { config, shutdown_tx: None, handle: None }
    }

    pub fn startup(&mut self) -> io::Result<()> {
        if self.handle.is_some() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "server already running"));
        }
        if self.config.tick_rate == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "tick_rate must be > 0"));
        }

        let runtime = Builder::new_multi_thread().enable_all().build()?;
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let config = self.config.clone();

        let handle = std::thread::Builder::new()
            .name("game-server".into())
            .spawn(move || {
                runtime.block_on(async move {
                    if let Err(e) = run_server(config, shutdown_rx).await {
                        log::error!("[game-server] fatal error: {e}");
                    }
                });
            })?;

        self.shutdown_tx = Some(shutdown_tx);
        self.handle = Some(handle);
        log::info!("[game-server] spawned on background thread");
        Ok(())
    }

    pub fn shutdown(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

impl Drop for GameServer {
    fn drop(&mut self) {
        self.shutdown();
    }
}

async fn run_server(
    config: GameServerConfig,
    mut shutdown_rx: oneshot::Receiver<()>,
) -> anyhow::Result<()> {
    let network = Arc::new(NetworkServer::create(
        false,
        config.port,
    ).await?);

    let mut ticker = interval(Duration::from_secs_f64(1.0 / config.tick_rate as f64));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

    log::info!(
        "[game-server] game loop starting at {} Hz on {:?}",
        config.tick_rate,
        network.local_addr(),
    );

    let mut total_packets: u64 = 0;

    loop {
        // checks if server has shutdown
        tokio::select! {
            _ = &mut shutdown_rx => {
                log::info!("[game-server] shutting down");
            }
            _ = ticker.tick() => {}
        }

        let packets = network.poll();
        if packets.is_empty() {
            continue;
        }

        for packet in &packets {
            total_packets += 1;
            println!(
                "[recv] client({}) {} bytes: {:?}",
                packet.client_id,
                packet.data.len(),
                packet.data,
            );
        }

        network.broadcast(&total_packets.to_le_bytes());
    }

    drop(network);
    return Ok(());
}
