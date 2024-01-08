use serde::{Deserialize, Serialize};


use strum_macros::{Display, EnumIter, EnumString};

use std::sync::atomic::Ordering;

use crate::config::Config;
use crate::music::song::Song;
use crate::STOP_FLAG;
use color_eyre::Result;

use log::{debug, error, trace, warn};

use crate::audio::AudioTask;
use crate::router::{start_router, wait, RouterTask};

pub struct Server {
    pub config: Config,
    pub audio_task: Option<AudioTask>, // this task has for only role to send queued songs to the audio thread
    // when it finishes playing a song
    pub router_task: Option<RouterTask>,
}

impl Server {
    pub fn new(config: &Config) -> Self {
        Server {
            config: config.clone(),
            audio_task: None,
            router_task: None,
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

