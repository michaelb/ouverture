use log::debug;
use std::net::SocketAddr;
use tokio::{
    net::{TcpListener, TcpStream},
    task::JoinHandle,
};

use crate::server::Server;
use std::sync::{Arc, Mutex};

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub struct RouterTask {
    pub addr: SocketAddr,
    handle: JoinHandle<()>,
    stop: Arc<Mutex<bool>>,
}

pub async fn start_router() -> RouterTask {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();

    let addr = listener.local_addr().unwrap().clone();

    let handle = tokio::spawn(async move { router(listener).await });

    RouterTask {
        addr,
        handle,
        stop: Arc::new(Mutex::new(false)),
    }
}

pub async fn stop_router(router: RouterTask) {
    let flag = router.stop;
    *flag.lock().unwrap() = true;
    router.handle.await.unwrap()
}

async fn router(listener: TcpListener) -> () {
    debug!("launched API router");

    let app = Router::new().route("/", get(root));
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
