use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ouverture", about = "A next-gen music player")]
pub struct Opt {
    /// Log level
    #[structopt(long = "log-level")]
    pub log_level: Option<String>,

    /// Log destination, stderr by default
    #[structopt(long = "log-destination")]
    pub log_destination: Option<PathBuf>,

    /// Config path
    #[structopt(short, long)]
    pub config: Option<PathBuf>,
}
