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

    pub external_server: bool,
}

impl Config {
    pub fn new(path: &Path) -> Result<Config> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let all_config = contents.parse::<toml::Table>()?;

        let config = Config::toml_table_to_config(&all_config)?;
        Ok(config)
    }

    fn toml_table_to_config(t: &toml::Table) -> Result<Config> {
        let mut config = Config {
            server_address: String::from("127.0.0.1"),
            theme: String::from("light"),
            server_port: 6603,
            external_server: true,
        };

        if let Some(toml::Value::Boolean(external_server)) = t.get("external_server") {
            config.external_server = *external_server;
        }

        if let Some(toml::Value::Integer(server_port)) = t.get("server_port") {
            config.server_port = *server_port as usize;
        }

        if let Some(toml::Value::String(server_address)) = t.get("server_address") {
            config.server_address = server_address.clone();
        }

        if let Some(toml::Value::String(theme)) = t.get("theme") {
            config.theme = theme.clone();
        }

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            server_address: "127.0.0.1".to_string(),
            server_port: 6603,
            theme: String::from("light"),
            external_server: false,
        }
    }
}
