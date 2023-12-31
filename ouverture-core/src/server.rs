use async_stream::try_stream;
use bincode;
use futures_core::Future;
use futures_core::stream::Stream;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::SocketAddrV4;
use std::sync::{Arc, Mutex};
use structopt::lazy_static::lazy_static;
use strum_macros::{Display, EnumIter, EnumString};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use std::sync::atomic::Ordering;

use crate::config::Config;
use crate::error::ServerError;
use crate::{library::*, STOP_FLAG};
use crate::music::song::Song;
use color_eyre::Result;
use std::pin::Pin;

use crate::audio::AudioState;

use log::{debug, error, info, trace, warn};
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

    async fn handle_command(
        command: Command,
        config: Config,
        audio_state: Arc<Mutex<AudioState>>,
        mut socket: &mut TcpStream,
    ) {
        match command {
            Command::Play(opt_song) => audio_state.lock().unwrap().play(opt_song),
            Command::Toggle => audio_state.lock().unwrap().toggle(),

            Command::Pause => audio_state.lock().unwrap().pause(),
            Command::Enqueue(song) => audio_state.lock().unwrap().enqueue(song),

            Command::Next => audio_state.lock().unwrap().next(),
            Command::Previous => audio_state.lock().unwrap().previous(),

            Command::Scan => scan(&config).await,
            Command::GetList(i) => {
                let list = list(&config, i).await;
                match Self::reply(Reply::List(list), &mut socket).await {
                    Ok(_) => trace!("Replied 'list' successfully"),
                    Err(e) => {
                        warn!("Failed to send 'list' reply to client: {:?}", e)
                    }
                }
            }

            Command::GetCurrentSong => {
                let current_song = audio_state.lock().unwrap().current_song.clone();
                let current_seek = AudioState::get_seek(audio_state).await;
                match Self::reply(
                    Reply::CurrentSong(current_song.clone(), current_seek),
                    &mut socket,
                )
                .await
                {
                    Ok(_) => trace!(
                        "Replied 'current song is {current_song:?}' (seek = {}%) successfully",
                        current_seek * 100.0
                    ),
                    Err(e) => {
                        warn!("Failed to send 'list' reply to client: {:?}", e)
                    }
                }
            }

            Command::Seek(seek) => audio_state.lock().unwrap().set_seek(seek),

            Command::Ping => (),
            Command::Restart => (),
            Command::Stop => {
                // state.lock().unwrap().stop = true;
            }
        };
    }

    // clients can use this helper to send commands to a server
    // and wait till the server replies 'done'
    pub async fn send_wait(
        message: &Command,
        address: &str,
    ) -> Result<Reply, Box<dyn Error + Send + Sync>> {
        let encoded: Vec<u8> = message.prepare_query()?;
        let mut stream = TcpStream::connect(address).await?;
        stream.write_all(&encoded).await?;

        // Wait for 'done', displaying all other replies
        let mut buf = [0u8; 8];
        loop {
            match stream.read(&mut buf).await {
                // socket closed
                Ok(n) if n == 0 => break,
                Ok(n) => n,
                Err(e) => {
                    error!("failed to read from socket; err = {:?}", e);
                    break;
                }
            };

            let size = match Reply::decode_size(buf) {
                Ok(i) => i,
                Err(e) => {
                    trace!("error: {:?}, stopping", e);
                    break;
                }
            };

            let mut payload = vec![0; size as usize];
            let res = stream.read_exact(&mut payload[..]).await;
            trace!("res from socket = {:?}", res);

            let decoded_reply = bincode::deserialize::<Reply>(&payload);
            if let Ok(ok_reply) = decoded_reply {
                //only return the meaningful value
                match ok_reply {
                    Reply::Received(_) => continue,
                    _ => return Ok(ok_reply),
                }
            }
        }
        Err("Communication with the server failed".into())
    }

    // sync-callable version that creates its own tokio runtime
    // to wait on tcp response stuff
    pub fn send_wait_sync(
        message: &Command,
        address: &str,
    ) -> Result<Reply, Box<dyn Error + Send + Sync>> {
        let rt = Runtime::new().unwrap();
        rt.block_on(Server::send_wait(message, address))
    }

    // send a command to the server and receive an stream to listen to the server's responses
    pub async fn send<'a>(
        message: &'a Command,
        address: &'a str,
    ) -> impl Stream<Item = Result<Reply, Box<dyn Error + Send + Sync>>> + 'a {
        try_stream! {
              let encoded: Vec<u8> = message.prepare_query()?;
              let mut stream = TcpStream::connect(address).await?;
              stream.write_all(&encoded).await?;
               // Wait for 'done', yielding all other replies
              let mut buf = [0u8; 8];
              loop {
                  match stream.read(&mut buf).await {
                      // socket closed
                      Ok(n) if n == 0 => break,
                      Ok(n) => n,
                      Err(e) => {
                          error!("failed to read from socket; err = {:?}", e);
                          break;
                      }
                  };

        let size = match Reply::decode_size(buf){
                  Ok(i) => i,
                  Err(e) => {trace!("error: {:?}, stopping",e); break;},
              };


                  let mut payload = vec![0; size as usize];
                  let res = stream.read_exact(&mut payload[..]).await;
                  trace!("res from socket = {:?}", res);

                  let decoded_reply = bincode::deserialize::<Reply>(&payload)?;
               match decoded_reply {
                   Reply::Done => { yield Reply::Done; break },
                   r => yield r,
               }


              }

          }
    }

    async fn reply(
        reply: Reply,
        stream: &mut TcpStream,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let encoded_reply: Vec<u8> = reply.prepare_query()?;
        stream.write_all(&encoded_reply).await?;
        Ok(())
    }
    // async fn reply(reply: Reply, address: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    //        let encoded_reply: Vec<u8> = reply.prepare_query()?;
    //        let mut stream = TcpStream::connect(address).await?;
    //        stream.write_all(&encoded_reply).await?;
    //        Ok(())
    //    }
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
