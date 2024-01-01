use axum::extract::rejection::JsonRejection;
use axum::extract::{Json, State};
use axum::routing::{get, post};
use axum::response::{Response, IntoResponse};
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
            .route("/next", get(next))
            .route("/previous", get(previous))
            .route("/scan", get(scan))
            .route("/enqueue", post(enqueue))
            .route("/seek", post(seek))
            .route("/list", get(getlist))
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

async fn seek(State(server): State<&Server>, payload: Json<f32>) {
    let Json(seek) = payload;
    server
        .audio_task
        .as_ref()
        .unwrap()
        .state
        .lock()
        .unwrap()
        .set_seek(seek);
}

async fn getlist(State(server): State<&Server>, payload: Option<Json<String>>) -> Response {
    let list = if let Some(payload) = payload {
        let Json(opt_string) = payload;
        crate::library::list(&server.config, Some(opt_string)).await
    } else {
        crate::library::list(&server.config, None).await
    };
    Json::from(list).into_response()
}

async fn enqueue(State(server): State<&Server>, payload: Json<Song>) {
    let Json(song) = payload;
    server
        .audio_task
        .as_ref()
        .unwrap()
        .state
        .lock()
        .unwrap()
        .enqueue(song);
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

async fn scan(State(server): State<&Server>) {
    crate::library::scan(&server.config).await;
}
