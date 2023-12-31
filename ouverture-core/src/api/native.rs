use axum::Router;
use axum::routing::{get,post};

use log::info;

pub struct Native {

}

impl Native {
    pub fn route() -> Router {
        Router::new().route("/", get(root)).route("/play", get(play))
    }
}


async fn root() -> &'static str {
    "Hello, native!"
}

async fn play() {
    info!("try to play");
    // server!().audio_task.as_ref().unwrap().state.lock().unwrap().play(None);

}

