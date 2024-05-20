use serde::{Deserialize, Serialize};

use strum_macros::{Display, EnumIter, EnumString};

use std::path::PathBuf;
use std::sync::atomic::Ordering;

use crate::config::Config;
use crate::database::song;
use crate::music::song::Song;
use crate::STOP_FLAG;
use color_eyre::Result;
use itertools::Itertools;

use log::{debug, error, trace, warn};

use crate::audio::AudioTask;
use crate::router::{start_router, wait, RouterTask};

use sea_orm::entity::prelude::*;
use sea_orm::Database;

pub struct Server {
    pub config: Config,
    pub db: DatabaseConnection,
    pub audio_task: Option<AudioTask>, // this task has for only role to send queued songs to the audio thread
    // when it finishes playing a song
    pub router_task: Option<RouterTask>,
}

impl Server {
    pub async fn new(config: &Config) -> Self {
  let database_url = "postgres://ouverture:ouverture@localhost:".to_string()
            + &config.database_port.to_string()
            + "/ouverture";
        Server {
            config: config.clone(),
            audio_task: None,
            router_task: None,
            db: Database::connect(&database_url).await.unwrap()
        }
    }

    pub fn stop() {
        debug!("Stopping command received");
        STOP_FLAG.store(true, Ordering::Relaxed);
    }

    pub async fn run(&mut self) -> Result<()> {
        let address =
            self.config.server_address.clone() + ":" + &self.config.server_port.to_string();

        self.audio_task = Some(AudioTask::run());

        let server_ref_for_router_ops;
        unsafe {
            server_ref_for_router_ops = &mut *(self as *mut Server);
        } // unsafe borrow, but we won't be badly using self here until the router and all other
          // stuff is dropped
        let r = Some(start_router(&address, server_ref_for_router_ops).await);
        self.router_task = r;

        wait(self.router_task.as_mut().unwrap()).await;

        // by this point the router and all API stuff MUST be dropped !

        Ok(())
    }

    pub fn music_folders(&self) -> Vec<PathBuf> {
        self.config.library.clone()
    }

    pub async fn list_songs(&self) -> Vec<Song> {
        let song_found: Vec<song::Model> = song::Entity::find().all(&self.db).await.unwrap();
        song_found.into_iter().map(|m| Song::from(m)).collect()
    }
    pub async fn list_artists(&self) -> Vec<String> {

        let song_found: Vec<song::Model> = song::Entity::find().all(&self.db).await.unwrap();
        song_found.into_iter().map(|m| Song::from(m).artist).filter(|os| os.is_some()).map(|os| os.unwrap()).unique().collect()
    }
}

#[non_exhaustive]
#[derive(Display, Debug, Serialize, Deserialize, EnumString, EnumIter, Clone)]
pub enum Command {
    // "Music" commands
    Play(Option<Song>),
    Pause,
    Toggle,
    Next,
    Previous,
    Enqueue(Song),
    Seek(f32),

    // "Library" commands
    Scan,

    // "Get info" commands
    GetList(Option<String>),
    GetCurrentSong,

    // "Server" commands
    Ping,
    Restart,
    Stop,
}
