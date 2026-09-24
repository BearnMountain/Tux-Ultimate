mod engine;
mod game;
mod util;
mod headless;
mod app;

// internal libs
use util::config::Config;
use app::App;

// external libs
use env_logger::Env;
use winit::{event_loop::{ControlFlow, EventLoop}};
use clap::Parser;
use core::panic;
use std::path::{Path, PathBuf};

/*
myapp server --host 0.0.0.0 --port 7777
myapp server --headless --host 0.0.0.0 --port 7777

myapp client --connect 192.168.1.10:7777
*/

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    headless: bool,

    #[arg(long)]
    headed: bool,

    #[arg(short = 'c', long)]
    config: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init system info
    env_logger::Builder::from_env(Env::default().default_filter_or("warn,app=debug")).init();
    
    let args = Args::parse();

    if (!args.headless && !args.headed) || 
        (args.headless && args.headed){
        panic!("Must define if app is to be run as --headless or --headed");
    }
    
    match args.config {
        Some(path) => Config::init(path.as_path()),
        None => Config::init(Path::new("assets/config.toml")),
    };

    if args.headless && !args.headed {
        println!("Running program as headless");
    }

    if !args.headless && args.headed {
        let rt = tokio::runtime::Runtime::new().expect("failed to start app");
        let _guard = rt.enter();

        let event_loop = EventLoop::new()?;

        event_loop.set_control_flow(ControlFlow::Poll); // preferable for games

        let mut app = App::new();
        event_loop.run_app(&mut app)?;
    }

    return Ok(());
}
