use log::debug;
use std::{net::SocketAddr, sync::atomic::Ordering};
use tokio::{
    net::{TcpListener, TcpStream},
    task::JoinHandle,
};
use tower_http::timeout::TimeoutLayer;

use crate::server::Server;
use crate::STOP_FLAG;
use std::sync::{Arc, Mutex};

use tokio::time::{self, Duration, Instant};

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::api::native::Native;

pub struct RouterTask {
    pub addr: SocketAddr,
    pub handle: Option<JoinHandle<()>>,
}

pub async fn start_router(address: &str, server: &'static mut Server) -> RouterTask {
    let listener = TcpListener::bind(address).await.unwrap();

    let addr = listener.local_addr().unwrap().clone();


    let handle = tokio::spawn(async move { router(listener, server).await });

    RouterTask {
        addr,
        handle: Some(handle),
    }
}

pub async fn wait(router: &mut RouterTask) {
    return router.handle.take().unwrap().await.unwrap();
}

async fn router(listener: TcpListener, server: &Server) -> () {
    debug!("launched API router");

    let native_api = Native::route();
    let api_routes = Router::new()
        // .nest("/native", user_routes)
        .nest("/native", native_api);

    let app = Router::new().route("/", get(root)).nest("/api", api_routes);
    axum::serve(listener, app).with_graceful_shutdown(signal()).await.unwrap();
    debug!("routing exited");
}

const SHUTDOWN_SIGNAL_POLL_FREQ_MS : u64 = 100;

async fn signal(){
    let mut interval = time::interval(Duration::from_millis(SHUTDOWN_SIGNAL_POLL_FREQ_MS));
    loop {
        if STOP_FLAG.load(Ordering::Relaxed) {
            return;
        }
        interval.tick().await;

    }
}


async fn root() -> &'static str {
    "Router root"
}
