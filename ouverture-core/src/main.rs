mod config;
mod opt;
mod server;
use crate::server::Server;
use chrono;
use color_eyre::eyre::eyre;
use color_eyre::{eyre::Report, eyre::WrapErr, Result, Section};
use fern::colors::{Color, ColoredLevelConfig};
use log::{debug, error, info, trace, warn};
use opt::Opt;
use std::borrow::Borrow;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let opts = Opt::from_args();
    match opts.log_destination.clone() {
        None => setup_logger(LogDestination::StdErr)?,
        Some(path) => setup_logger(LogDestination::File(path))?,
    };
    info!("Opts = {:?}", opts);
    // let config = config::Config::new_from_file(config_path).unwrap();
    let config = config::Config::default();
    info!("Config : {:?}", config);

    let res = Server::start().await;
    Ok(())
}

#[derive(Debug)]
enum LogDestination {
    File(PathBuf),
    StdErr,
}

fn setup_logger(dest: LogDestination) -> Result<()> {
    let colors = ColoredLevelConfig::default();
    let res = match dest {
        LogDestination::File(path) => fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
                    color_line =
                        format_args!("\x1B[{}m", colors.get_color(&record.level()).to_fg_str()),
                    date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    target = record.target(),
                    level = colors.color(record.level()),
                    message = message,
                ))
            })
            .level(log::LevelFilter::Trace)
            .chain(fern::log_file(path)?)
            .apply(),
        LogDestination::StdErr => fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
                    color_line =
                        format_args!("\x1B[{}m", colors.get_color(&record.level()).to_fg_str()),
                    date = chrono::Local::now().format("%H:%M:%S"),
                    target = record.target(),
                    level = colors.color(record.level()),
                    message = message,
                ))
            })
            .level(log::LevelFilter::Trace)
            .chain(std::io::stderr())
            .apply(),
    };
    warn!("setup logging: {:?}", res);
    match res {
        Ok(_) => Ok(()),
        Err(_) => Err(eyre!(
            "Failed to set up the logger, is the log destination valid ?"
        )),
    }
}
