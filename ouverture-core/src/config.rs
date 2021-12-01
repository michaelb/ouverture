use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use toml;

use color_eyre::{eyre::eyre, Result};

#[derive(Deserialize, Debug)]
pub struct Config {
    library: Vec<PathBuf>,
}

impl Config {
    pub fn new_from_file(path: &Path) -> Result<Config> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Config {
        Config { library: vec![] }
    }
}
