pub mod config;
pub mod database;
pub mod music;
pub mod library;
pub mod server;

use config::Config;
use server::Server;
use std::{error::Error, path::Path};

use database::*;

use color_eyre::eyre::eyre;
use color_eyre::Result;
use log::{error, info, debug};

pub async fn start(config: Config) -> Result<(), Box<dyn Error>> {
    debug!("Ouverture server started");
    let address = config.server_address.clone() + ":" + &config.server_port.clone();
    let mut pg = setup_db(config.clone()).await?;
    start_db(&mut pg, config.clone()).await?;
    // test_db(config).await;

    let server_exit_status = Server::start(&address).await;

    library::scan(&config.library[0]);

    info!("stopping database");
    let res = pg.stop_db().await;
    match res {
        Err(e) => error!("failed to stop database {:?}", e),
        _ => (),
    };

    debug!("Ouverture server stopped");
    server_exit_status
}
