mod config;
mod music;
mod opt;
mod server;
use crate::server::{Command::Stop, Server};
use chrono;
use color_eyre::eyre::eyre;
use color_eyre::{eyre::Report, eyre::WrapErr, Result, Section};
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter::*;
use log::{debug, error, info, trace, warn};
use opt::Opt;
use structopt::StructOpt;

use ouverture_core::config::Config;
use ouverture_core::start;

mod logger;
use logger::{setup_logger, LogDestination::*};
use futures::stream::StreamExt;
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;

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
    let address = config.server_address.clone() + ":" + &config.server_port.clone();
    let signals = Signals::new(&[
        SIGTERM,
        SIGINT,
        SIGQUIT,
    ])?;
    let handle = signals.handle();
    let signals_task = tokio::spawn(handle_signals(signals, address));

    // Start ouverture server
    let res = start(config).await;

    handle.close();
    signals_task.await?;


    match res {
        Err(e) => Err(eyre!(format!("{:?}", e))),
        Ok(_) => Ok(()),
    }
}

async fn handle_signals(signals: Signals, address: String) {
    let mut signals = signals.fuse();
    while let Some(signal) = signals.next().await {
        match signal {
            SIGTERM | SIGINT | SIGQUIT => {
                // Shutdown the system;
                Server::send(&Stop, &address).await.unwrap();

            },
            _ => unreachable!(),
        }
    }
}
