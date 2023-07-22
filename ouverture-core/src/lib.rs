pub mod audio;
pub mod config;
pub mod database;
pub mod library;
pub mod logger;
pub mod music;
pub mod server;

use config::Config;
use server::Server;

use std::sync::{Arc, Mutex};

use server::{Command::Stop};

use futures::stream::StreamExt;

use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use database::*;

use color_eyre::{eyre::eyre, Result};
use log::{debug, error, info, warn, trace};

use daemonize::Daemonize;

#[tokio::main]
pub async fn start_with_handlers(config: Config) -> Result<()> {
   // Set up signal handlers
    let first_signal = Arc::new(Mutex::new(true));
    let address = config.server_address.clone() + ":" + &config.server_port.to_string();
    let signals = Signals::new(&[SIGTERM, SIGINT, SIGQUIT])?;
    let handle = signals.handle();

    let signals_task = tokio::spawn(handle_signals(signals, address, first_signal.clone()));


    // Start ouverture server (unique async entry point)
    let res = start(config).await;
    let return_value = match res {
        Err(e) => {
            warn!("Server exited with an error: {:?}", e);
            Err(eyre!(format!("{:?}", e)))
        }
        _ => Ok(()),
    };

    handle.close();
    signals_task.await?;

    // If a stop signal was received, exit with non-zero status
    if !*first_signal.lock().unwrap() {
        std::process::exit(1);
    }
    return return_value;

}

pub async fn start(config: Config) -> Result<()> {
    trace!("asked for ouverture server start");

    info!("Ouverture server started");

    //encase ouverture within a scope, so that everything is dropped before the final "stopped"
    let status = {
        info!("setupping db, config: {:?}", config);
        let mut pg = setup_db(config.clone()).await?;
        trace!("db setup");

        let res = start_db(&mut pg, config.clone()).await;
        trace!("db up: {res:?}");
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

        trace!("database up");
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

async fn handle_signals(signals: Signals, address: String, first_signal: Arc<Mutex<bool>>) {
    let mut signals = signals.fuse();
    while let Some(signal) = signals.next().await {
        match signal {
            SIGTERM | SIGINT | SIGQUIT => {
                // Shutdown the system;
                info!("signal received, shutting down");
                if *first_signal.lock().unwrap() {
                    *first_signal.lock().unwrap() = false;
                    let _ = Server::send_wait(&Stop, &address).await;
                } else {
                    std::process::exit(signal);
                }
            }
            _ => unreachable!(),
        }
    }
}
