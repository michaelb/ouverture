use async_stream::try_stream;
use bincode;
use futures_core::stream::Stream;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::{Arc, Mutex};
use strum_macros::{Display, EnumIter, EnumString};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::config::Config;
use crate::library::*;
use crate::music::song::Song;
use color_eyre::Result;

use rc_event_queue::mpmc::EventQueue;

use crate::audio::AudioState;

use log::{debug, error, info, trace, warn};
use std::pin::Pin;
use tokio::sync::broadcast::{channel, Receiver, Sender};

use crate::audio::{
    audio_thread_pause, audio_thread_play, audio_thread_play_song, start_audio_thread,
    stop_audio_thread, AudioTask,
};

pub struct Server {
    config: Config,
    audio_task: Option<AudioTask>, // this task has for only role to send queued songs to the audio thread
                                   // when it finishes playing a song
    state: Arc<Mutex<ServerState>>,
}

/// unique shareable / queryable state
#[derive(Clone)]
struct ServerState {
    stop: bool, // stop-the-server flag
}

impl Server {
    pub fn new(config: &Config) -> Self {
        let state = Arc::new(Mutex::new(ServerState { stop: false }));
        Self {
            config: config.clone(),
            audio_task: None,
            state,
        }
    }

    pub async fn run(mut self) -> Result<()> {
        let address =
            self.config.server_address.clone() + ":" + &self.config.server_port.to_string();
        trace!("Starting TCP server on {:?}", &address);
        let listener = TcpListener::bind(&address).await?;
        trace!("Server bound to tcp port");

        self.audio_task = Some(AudioTask::run());
        let audio_state = self.audio_task.as_ref().unwrap().state.clone();

        // accept many clients at the same time
        let res = loop {
            let (mut socket, _) = listener.accept().await?;
            let client_address = socket.peer_addr()?;
            let client_address = format!("{}", client_address);
            debug!("New client: {}", client_address);

            let state = self.state.clone();
            let audio_state = audio_state.clone();

            let config = self.config.clone();
            let handle = tokio::spawn(async move {
                let mut buf = [0u8; 8];

                // In a loop, read all the data from the socket
                loop {
                    debug!("waiting for new packet");
                    match socket.read(&mut buf).await {
                        // socket closed
                        Ok(n) if n == 0 => break,
                        Ok(n) => n,
                        Err(e) => {
                            debug!("Client disconnected : {}", e);
                            break;
                        }
                    };

                    debug!("got new packet");
                    let size = u64::from_ne_bytes(buf);

                    let mut payload = vec![0; size as usize];
                    let res = socket.read_exact(&mut payload[..]).await;
                    trace!("res from socket = {:?}", res);

                    let decoded_command = bincode::deserialize::<Command>(&payload);
                    match decoded_command {
                        Ok(command) => {
                            info!("{command} command received");
                            let res =
                                Self::reply(Reply::Received(command.to_string()), &mut socket)
                                    .await;
                            trace!("Replied 'received': status: {:?}", res);
                            Self::handle_command(
                                command,
                                config.clone(),
                                state.clone(),
                                audio_state.clone(),
                                &mut socket,
                            )
                            .await;

                            match Self::reply(Reply::Done, &mut socket).await {
                                Ok(_) => trace!("Replied 'done' successfully"),
                                Err(e) => warn!("Failed to send 'done' to client: {:?}", e),
                            }

                            if state.lock().unwrap().stop {
                                break;
                            }
                        }
                        Err(e) => warn!("failed to decode message payload; err = {:?}", e),
                    };
                }

                trace!("Terminating tokio thread allocated to this request");
            });

            trace!("Waiting on tokio thread join for shutdown...");
            tokio::join!(handle).0?;

            // in case the Stop command was received, exit the loop.
            // The binded address is released at 'listener' drop
            if self.state.lock().unwrap().stop {
                break Ok(());
            }
        };

        self.audio_task.unwrap().stop();

        return res;
    }

    async fn handle_command(
        command: Command,
        config: Config,
        state: Arc<Mutex<ServerState>>,
        audio_state: Arc<Mutex<AudioState>>,
        mut socket: &mut TcpStream,
    ) {
        match command {
            Command::Play(i) => audio_state.lock().unwrap().play(i),
            Command::Toggle => audio_state.lock().unwrap().toggle(),

            Command::Pause => audio_state.lock().unwrap().pause(),

            Command::Next => (),
            Command::Previous => (),

            Command::Scan => scan(&config).await,
            Command::List(i) => {
                let list = list(&config, i).await;
                match Self::reply(Reply::List(list), &mut socket).await {
                    Ok(_) => trace!("Replied 'list' successfully"),
                    Err(e) => {
                        warn!("Failed to send 'list' reply to client: {:?}", e)
                    }
                }
            }

            Command::Ping => (),
            Command::Restart => (),
            Command::Stop => {
                state.lock().unwrap().stop = true;
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

            let size = u64::from_ne_bytes(buf);

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

                let size = u64::from_ne_bytes(buf);

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

    // "Library" commands
    Scan,
    List(Option<String>),

    // "Server" commands
    Ping,
    Restart,
    Stop,
}

#[non_exhaustive]
#[derive(Display, Debug, Serialize, Deserialize, EnumString, EnumIter)]
pub enum Reply {
    Received(String),
    List(Vec<Song>),
    Done,
}

impl Command {
    fn prepare_query(&self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        // create a 8-bytes prefix: the length of the whole (prefix+message)
        match bincode::serialized_size(self) {
            Ok(size) => {
                let mut message: Vec<u8> = (size as u64).to_ne_bytes().to_vec();
                // add the serialized content to the message
                message.extend(bincode::serialize(self).unwrap());
                Ok(message)
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}

impl Reply {
    fn prepare_query(&self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        // create a 8-bytes prefix: the length of the whole (prefix+message)
        match bincode::serialized_size(self) {
            Ok(size) => {
                let mut message: Vec<u8> = (size as u64).to_ne_bytes().to_vec();
                // add the serialized content to the message
                message.extend(bincode::serialize(self).unwrap());
                Ok(message)
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}
