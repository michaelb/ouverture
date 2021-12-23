mod config;
mod opt;
mod server;
mod music;
use crate::server::Server;
use chrono;
use color_eyre::eyre::eyre;
use color_eyre::{eyre::Report, eyre::WrapErr, Result, Section};
use fern::colors::{Color, ColoredLevelConfig};
use log::{debug, error, info, trace, warn};
use opt::Opt;
use structopt::StructOpt;
use log::LevelFilter::*;

use ouverture_core::start;
use ouverture_core::config::Config;

mod logger;
use logger::{setup_logger, LogDestination::*};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let opts = Opt::from_args();
    let level = match opts.log_level.as_deref() {
        None => Info,
        Some("info") => Info,
        Some("warn") => Warn,
        Some("trace") => Trace,
        Some("error") => Error,
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
