use color_eyre::eyre::eyre;
use color_eyre::{eyre::Report, eyre::WrapErr, Result, Section};
use fern::colors::{Color, ColoredLevelConfig};
use log::debug;
use std::path::{Path, PathBuf};
#[derive(Debug)]
pub enum LogDestination {
    File(PathBuf),
    StdErr,
}

pub fn setup_logger(dest: LogDestination, level: log::LevelFilter) -> Result<()> {
    // hide the 'info' from dependencies (postgresql, sqlx) when in 'info' log level
    let level_for_dependencies = match level {
        log::LevelFilter::Info => log::LevelFilter::Warn,
        _ => level,
    };
    let colors = ColoredLevelConfig::default().debug(Color::Magenta);
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
            .level(level)
            .level_for("pg_embed", level_for_dependencies)
            .level_for("sqlx", level_for_dependencies)
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
            .level(level)
            .level_for("pg_embed", level_for_dependencies)
            .level_for("sqlx", level_for_dependencies)
            .chain(std::io::stderr())
            .apply(),
    };
    debug!("setup logging: {:?}", res);
    match res {
        Ok(_) => Ok(()),
        Err(_) => Err(eyre!(
            "Failed to set up the logger, is the log destination a valid path?"
        )),
    }
}
