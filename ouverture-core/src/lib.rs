pub mod config;
pub mod database;
pub mod library;
pub mod music;
pub mod server;

use config::Config;
use server::Server;
use std::{error::Error, path::Path};

use database::*;

use color_eyre::eyre::eyre;
use color_eyre::Result;
use log::{debug, error, info};

pub async fn start(config: Config) -> Result<(), Box<dyn Error>> {
    info!("Ouverture server started");
    let mut pg = setup_db(config.clone()).await?;
    start_db(&mut pg, config.clone()).await?;
    // test_db(config).await;

    let server_exit_status = Server::start(&config).await;


    debug!("stopping database");
    let res = pg.stop_db().await;
    match res {
        Err(e) => error!("failed to stop database {:?}", e),
        _ => (),
    };

    info!("Ouverture server stopped");
    server_exit_status
}
