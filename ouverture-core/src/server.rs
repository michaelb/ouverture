use async_stream::try_stream;
use bincode;

use futures_core::stream::Stream;
use serde::{Deserialize, Serialize};
use std::error::Error;

use std::sync::{Arc, Mutex};

use strum_macros::{Display, EnumIter, EnumString};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use std::sync::atomic::Ordering;

use crate::config::Config;
use crate::error::ServerError;
use crate::music::song::Song;
use crate::{library::*, STOP_FLAG};
use color_eyre::Result;

use crate::audio::AudioState;

use log::{debug, error, trace, warn};
use tokio::runtime::Runtime;

use crate::audio::AudioTask;
use crate::router::{start_router, wait, RouterTask};

// magic number to identify ouverture protocol on the wire
const MAGIC_ID_OUVERTURE_PROTOCOL: u64 = 0xACDE314152960000;

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

#[non_exhaustive]
#[derive(Display, Debug, Serialize, Deserialize, EnumString, EnumIter, Clone)]
pub enum Reply {
    Received(String),
    List(Vec<Song>),
    CurrentSong(Option<Song>, f32), // current song and current seek
    Done,
}

impl Command {
    fn prepare_query(&self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        // create a 8-bytes prefix: the length of the whole (prefix+message)
        match bincode::serialized_size(self) {
            Ok(size) => {
                if size > std::u32::MAX as u64 {
                    return Err(Box::new(ServerError::MessageTooBig));
                }

                let mut message: Vec<u8> =
                    (MAGIC_ID_OUVERTURE_PROTOCOL + size).to_ne_bytes().to_vec();
                // add the serialized content to the message
                message.extend(bincode::serialize(self).unwrap());
                Ok(message)
            }
            Err(e) => Err(Box::new(e)),
        }
    }
    fn decode_size(buf: [u8; 8]) -> Result<u16, Box<dyn Error + Send + Sync>> {
        let size = u64::from_ne_bytes(buf);
        if size >> 16 == MAGIC_ID_OUVERTURE_PROTOCOL >> 16 {
            return Ok((size - MAGIC_ID_OUVERTURE_PROTOCOL) as u16);
        } else {
            return Err(Box::new(ServerError::NotNativeProtocol));
        }
    }
}

impl Reply {
    fn prepare_query(&self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        // create a 8-bytes prefix: the length of the whole (prefix+message)
        match bincode::serialized_size(self) {
            Ok(size) => {
                if size > std::u16::MAX as u64 {
                    return Err(Box::new(ServerError::MessageTooBig));
                }

                let mut message: Vec<u8> =
                    (MAGIC_ID_OUVERTURE_PROTOCOL + size).to_ne_bytes().to_vec();
                // add the serialized content to the message
                message.extend(bincode::serialize(self).unwrap());
                Ok(message)
            }
            Err(e) => Err(Box::new(e)),
        }
    }
    fn decode_size(buf: [u8; 8]) -> Result<u16, Box<dyn Error + Send + Sync>> {
        let size = u64::from_ne_bytes(buf);
        if size >> 16 == MAGIC_ID_OUVERTURE_PROTOCOL >> 16 {
            return Ok((size - MAGIC_ID_OUVERTURE_PROTOCOL) as u16);
        } else {
            return Err(Box::new(ServerError::NotNativeProtocol));
        }
    }
}
