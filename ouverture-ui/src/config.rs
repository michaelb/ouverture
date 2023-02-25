use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use toml;

use color_eyre::Result;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub theme: String,
    pub server_address: String,
    pub server_port: usize,
}

impl Config {
    pub fn new(path: &Path) -> Result<Config> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            server_address: "127.0.0.1".to_string(),
            server_port: 6603,
            theme: String::from("light"),
        }
    }
}
