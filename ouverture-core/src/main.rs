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

mod logger;
use logger::{setup_logger, LogDestination::*};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let opts = Opt::from_args();
    match opts.log_destination.clone() {
        None => setup_logger(StdErr)?,
        Some(path) => setup_logger(File(path))?,
    };
    info!("Opts = {:?}", opts);
    // let config = config::Config::new_from_file(config_path).unwrap();
    let config = match opts.config {
        None => config::Config::default(),
        Some(path) => config::Config::new_from_file(&path)?,
    };
    info!("Config : {:?}", config);

    let address = "127.0.0.1:6603";
    let res = Server::start(&address).await;
    info!("Server exiting with status: {:?}", res);
    Ok(())
}
