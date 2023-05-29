pub mod audio;
pub mod config;
pub mod database;
pub mod library;
pub mod logger;
pub mod music;
pub mod server;

use config::Config;
use server::Server;

use database::*;

use color_eyre::Result;
use log::{debug, error, info, warn};

pub async fn start(config: Config) -> Result<()> {
    info!("Ouverture server started");

    //encase ouverture within a scope, so that everything is dropped before the final "stopped"
    let status = {
        let mut pg = setup_db(config.clone()).await?;

        let res = start_db(&mut pg, config.clone()).await;
        if let Err(e) = res {
            warn!(
                "Retrying to start the database (may happen when the last process was interrupted)"
            );
            debug!("failed to start the database once {e}");
            if format!("{:?}", e).contains("PgStartFailure") {
                // failed to start db, may be due to lockfile presence and interrupt of the last ouverture server
                // let's retry once
                pg = setup_db(config.clone()).await?;
                start_db(&mut pg, config.clone()).await?;
            }
        }
        // test_db(config).await;

        let server = Server::new(&config);
        let server_exit_status = server.run().await;

        debug!("stopping database");
        let res = pg.stop_db().await;
        match res {
            Err(e) => error!("failed to stop database {:?}", e),
            _ => (),
        };
        server_exit_status
    };

    info!("Ouverture server stopped");
    status
}
