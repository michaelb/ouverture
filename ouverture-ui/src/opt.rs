use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ouverture", about = "A next-gen music player")]
pub struct Opt {
    /// Log level filter, default to 'info'
    #[structopt(long = "log-level", possible_values(&["trace", "debug", "info", "warn", "error", "off"]))]
    pub log_level: Option<String>,

    /// Log destination, stderr by default
    #[structopt(long = "log-destination")]
    pub log_destination: Option<PathBuf>,

    /// Theme
    #[structopt(long = "theme")]
    pub theme: Option<String>,

    /// Config path
    #[structopt(short, long)]
    pub config: Option<PathBuf>,
}
