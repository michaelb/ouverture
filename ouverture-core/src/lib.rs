pub mod config;
pub mod database;
pub mod server;

use std::error::Error;
use config::Config;
use server::Server;

use database::{setup_db, start_db};

use color_eyre::eyre::eyre;
use color_eyre::Result;
use log::{error, info};

pub async fn start(config: Config) -> Result<(), Box<dyn Error>>  {
    let address = config.server_address.clone() + ":" + &config.server_port.clone();
    let mut pg = setup_db(config.clone()).await?;
    start_db(&mut pg, config).await?;

    let server_exit_status = Server::start(&address).await;

    info!("stopping database");
    let res = pg.stop_db().await;
    match res {
        Err(e) => error!("failed to stop database {:?}", e),
        _ => (),
    };

    server_exit_status

}
