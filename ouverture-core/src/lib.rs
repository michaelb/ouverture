pub mod config;
pub mod database;
pub mod server;

use config::Config;
use server::Server;

use color_eyre::eyre::eyre;
use color_eyre::Result;
use log::{debug, error, info, trace, warn};

pub async fn start(config: Config) -> Result<()> {
    let address = config.server_address + ":" + &config.server_port;
    let res = Server::start(&address).await;
    info!("Server exiting with status: {:?}", res);
    match res {
        Err(e) => Err(eyre!(format!("{:?}", e))),
        Ok(_) => Ok(()),
    }
}
