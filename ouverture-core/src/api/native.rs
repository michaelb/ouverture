use axum::extract::rejection::JsonRejection;
use axum::extract::{Json, State};
use axum::routing::{get, post};
use axum::Router;
use serde_json::Value;

use crate::music::song::Song;
use crate::server::Server;

use log::info;

pub struct Native {}

impl Native {
    pub fn route() -> Router<&'static Server> {
        Router::new()
            .route("/", get(root))
            .route("/play", get(play))
            .route("/toggle", get(toggle))
            .route("/pause", get(pause))
            .route("/next", get(pause))
            .route("/previous", get(pause))
    }
}

async fn root() -> &'static str {
    "Hello, native!"
}

async fn play(State(server): State<&Server>, payload: Option<Json<Song>>) {
    let opt_song = match payload {
        Some(Json(song)) => Some(song),
        None => None,
    };
    server
        .audio_task
        .as_ref()
        .unwrap()
        .state
        .lock()
        .unwrap()
        .play(opt_song);
}

async fn toggle(State(server): State<&Server>) {
    server
        .audio_task
        .as_ref()
        .unwrap()
        .state
        .lock()
        .unwrap()
        .toggle();
}

async fn pause(State(server): State<&Server>) {
    server
        .audio_task
        .as_ref()
        .unwrap()
        .state
        .lock()
        .unwrap()
        .pause();
}

async fn next(State(server): State<&Server>) {
    server
        .audio_task
        .as_ref()
        .unwrap()
        .state
        .lock()
        .unwrap()
        .next();
}

async fn previous(State(server): State<&Server>) {
    server
        .audio_task
        .as_ref()
        .unwrap()
        .state
        .lock()
        .unwrap()
        .previous();
}
