mod config;
mod music;
mod opt;
mod server;
use crate::server::Server;
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

#[tokio::main]
async fn main() -> Result<()> {
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

    start(config).await
}
