pub mod config;
pub mod database;
pub mod server;

use config::Config;
use server::Server;
use std::error::Error;

use database::setup_db;

use color_eyre::eyre::eyre;
use color_eyre::Result;
use log::{error, info};

pub async fn start(config: Config) -> Result<(), Box<dyn Error>> {
    let address = config.server_address + ":" + &config.server_port;
    let mut pg = setup_db().await?;
    pg.start_db().await?;

    let server_exit_status = Server::start(&address).await;

    info!("stopping database");
    let res = pg.stop_db().await;
    match res {
        Err(e) => error!("failed to stop database {:?}", e),
        _ => (),
    };

    server_exit_status
}
