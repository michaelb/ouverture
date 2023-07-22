extern crate ouverture_core;
mod opt;

use color_eyre::eyre::eyre;
use color_eyre::Result;
use futures::stream::StreamExt;
use log::LevelFilter::*;
use log::{debug, error, info, trace, warn};
use opt::Opt;
use ouverture_core::config::Config;
use ouverture_core::logger::{setup_logger, LogDestination::*};
use ouverture_core::start;
use structopt::StructOpt;

use daemonize::Daemonize;

fn main() -> Result<()> {
    // Setup color_eyre err manager

    color_eyre::install()?;

    let opts = Opt::from_args();
    debug!("Opts = {:?}", opts);

    let level = match opts.log_level.as_deref() {
        None => Info,
        Some("trace") => Trace,
        Some("debug") => Debug,
        Some("info") => Info,
        Some("warn") => Warn,
        Some("error") => Error,
        Some("off") => Off,
        Some(_) => Info, // unreachable because of the arg parser
    };

    match opts.log_destination.clone() {
        None => setup_logger(StdErr, level)?,
        Some(path) => setup_logger(File(path), level)?,
    };
    // let config = config::Config::new_from_file(config_path).unwrap();
    let mut config = match opts.config {
        None => Config::default(),
        Some(path) => Config::new_from_file(&path)?,
    };

    config.background = opts.background;

    if opts.background {
        let daemonize =
            Daemonize::new().working_directory(std::env::current_dir().unwrap_or("/tmp".into()));
        match daemonize.start() {
            Ok(_) => info!("Successfully forked ouverture-server process to the background"),
            Err(_) => error!("Failed to daemonize ouverture-server"),
        }
    }

    debug!("Config : {:?}", config);

    ouverture_core::start_with_handlers(config)
}
