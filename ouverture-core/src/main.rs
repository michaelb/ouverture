mod config;
mod logger;
mod music;
mod opt;
mod server;

use crate::server::{Command::Stop, Server};
use color_eyre::eyre::eyre;
use color_eyre::Result;
use futures::stream::StreamExt;
use log::LevelFilter::*;
use log::{info, warn};
use logger::{setup_logger, LogDestination::*};
use opt::Opt;
use ouverture_core::config::Config;
use ouverture_core::start;
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup color_eyre manager
    color_eyre::install()?;

    let opts = Opt::from_args();
    let level = match opts.log_level.as_deref() {
        None => Info,
        Some("trace") => Trace,
        Some("debug") => Debug,
        Some("info") => Info,
        Some("warn") => Warn,
        Some("error") => Error,
        Some("off") => Off,
        Some(_) => Info,
    };

    match opts.log_destination.clone() {
        None => setup_logger(StdErr, level)?,
        Some(path) => setup_logger(File(path), level)?,
    };
    info!("Opts = {:?}", opts);
    // let config = config::Config::new_from_file(config_path).unwrap();
    let config = match opts.config {
        None => Config::default(),
        Some(path) => Config::new_from_file(&path)?,
    };
    info!("Config : {:?}", config);

    // Set up signal handlers
    let first_signal = Arc::new(Mutex::new(true));
    let address = config.server_address.clone() + ":" + &config.server_port.to_string();
    let signals = Signals::new(&[SIGTERM, SIGINT, SIGQUIT])?;
    let handle = signals.handle();
    let signals_task = tokio::spawn(handle_signals(signals, address, first_signal.clone()));

    // Start ouverture server (unique async entry point)
    let res = start(config).await;
    let return_value = match res {
        Err(e) => {
            warn!("Server exited with an error: {:?}", e);
            Err(eyre!(format!("{:?}", e)))
        }
        _ => Ok(()),
    };

    handle.close();
    signals_task.await?;

    // If a stop signal was received, exit with non-zero status
    if !*first_signal.lock().unwrap() {
        std::process::exit(1);
    }
    return_value
}

async fn handle_signals(signals: Signals, address: String, first_signal: Arc<Mutex<bool>>) {
    let mut signals = signals.fuse();
    while let Some(signal) = signals.next().await {
        match signal {
            SIGTERM | SIGINT | SIGQUIT => {
                // Shutdown the system;
                if *first_signal.lock().unwrap() {
                    *first_signal.lock().unwrap() = false;
                    Server::send(&Stop, &address).await.unwrap();
                } else {
                    std::process::exit(signal);
                }
            }
            _ => unreachable!(),
        }
    }
}
