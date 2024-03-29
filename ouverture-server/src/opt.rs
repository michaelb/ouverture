use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ouverture-server",
    about = "A next-gen music player (server part)"
)]
pub struct Opt {
    /// Log level filter, default to 'info'
    #[structopt(long = "log-level", possible_values(&["trace", "debug", "info", "warn", "error", "off"]))]
    pub log_level: Option<String>,

    /// Log destination, stderr by default
    #[structopt(long = "log-destination")]
    pub log_destination: Option<PathBuf>,

    /// Config path
    #[structopt(short, long)]
    pub config: Option<PathBuf>,

    /// Config path
    #[structopt(short = "d", long = "daemonize")]
    pub background: bool,
}
