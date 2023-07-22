use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use toml;

use log::trace;

use color_eyre::Result;

use platform_dirs::AppDirs;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub library: Vec<PathBuf>,
    pub server_address: String,
    pub server_port: usize,

    pub background: bool,

    pub database_dir: PathBuf,
    pub database_port: usize,
}

impl Config {
    pub fn new_from_file(path: &Path) -> Result<Config> {
        trace!("start reading config from file path: {path:?}");
        trace!("read file ok ? : {:?}", File::open(path));
        let mut file = File::open(path)?;
        trace!("read file: {file:?}");
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        trace!("start reading config from file");

        let all_config = contents.parse::<toml::Table>()?;

        let config = Config::toml_table_to_config(&all_config)?;
        trace!("config is read ok: {config:?}");
        Ok(config)
    }

    fn toml_table_to_config(t: &toml::Table) -> Result<Config> {
        let mut config: Config = Default::default();

        if let Some(toml::Value::Boolean(background)) = t.get("background") {
            config.background = *background;
        }

        if let Some(toml::Value::Integer(server_port)) = t.get("server_port") {
            config.server_port = *server_port as usize;
        }

        if let Some(toml::Value::Integer(database_port)) = t.get("database_port") {
            config.database_port = *database_port as usize;
        }

        if let Some(toml::Value::String(server_address)) = t.get("server_address") {
            config.server_address = server_address.clone();
        }

        if let Some(toml::Value::String(database_dir)) = t.get("database_dir") {
            config.database_dir = PathBuf::from(database_dir);
        }

        if let Some(toml::Value::Array(library)) = t.get("library") {
            config.library = library
                .clone()
                .iter()
                .map(|v| PathBuf::from(v.as_str().unwrap_or("")))
                .collect();
        }

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            library: vec![],
            server_address: "127.0.0.1".to_string(),
            server_port: 6603,
            background: false,

            database_dir: AppDirs::new(Some("ouverture/postgres"), true)
                .unwrap()
                .data_dir,
            database_port: 6604,
        }
    }
}
