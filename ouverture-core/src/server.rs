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

use log::{debug, error, info, trace, warn};

pub struct Server {}

impl Server {
    pub async fn start(config: &Config) -> Result<(), Box<dyn Error>> {
        let address = config.server_address.clone() + ":" + &config.server_port.to_string();
        trace!("Starting TCP server on {:?}", &address);
        let listener = TcpListener::bind(&address).await?;
        trace!("Server bound to tcp port");

        let stop_flag = Arc::new(Mutex::new(false));

        // accept many clients at the same time
        loop {
            let local_stop_flag = stop_flag.clone();
            let (mut socket, _) = listener.accept().await?;
            let client_address = socket.peer_addr()?;
            let client_address = format!("{}", client_address);
            debug!("New client: {}", client_address);

            let config = config.clone();
            let handle = tokio::spawn(async move {
                let mut buf = [0u8; 8];

                // In a loop, read all the data from the socket
                loop {
                    match socket.read(&mut buf).await {
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
                    let res = socket.read_exact(&mut payload[..]).await;
                    trace!("res from socket = {:?}", res);

                    let decoded_command = bincode::deserialize::<Command>(&payload);
                    match decoded_command {
                        Ok(command) => {
                            info!("{command} command received");
                            let res =
                                Server::reply(Reply::Received(command.to_string()), &mut socket)
                                    .await;
                            trace!("Replied 'received': status: {:?}", res);
                            match command {
                                Command::Play(i) => (),
                                Command::Pause => (),
                                Command::Toggle => (),
                                Command::Next => (),
                                Command::Previous => (),

                                Command::Scan => scan(&config).await,
                                Command::List(i) => {
                                    let list = list(&config, i).await;
                                    match Server::reply(Reply::List(list), &mut socket).await {
                                        Ok(_) => trace!("Replied 'list' successfully"),
                                        Err(e) => {
                                            warn!("Failed to send 'list' reply to client: {:?}", e)
                                        }
                                    }
                                }

                                Command::Ping => (),
                                Command::Restart => (),
                                Command::Stop => {
                                    let mut flag = local_stop_flag.lock().unwrap();
                                    *flag = true;
                                    break;
                                }
                            };

                            match Server::reply(Reply::Done, &mut socket).await {
                                Ok(_) => trace!("Replied 'done' successfully"),
                                Err(e) => warn!("Failed to send 'done' to client: {:?}", e),
                            }
                        }
                        Err(e) => warn!("failed to decode message payload; err = {:?}", e),
                    };
                }
                trace!("Terminating tokio thread");
            });
            trace!("Waiting on tokio thread join for shutdown...");
            let res = tokio::join!(handle);

            // in case the Stop command was received, exit the loop.
            // The binded address is released at 'listener' drop
            if *stop_flag.lock().unwrap() {
                break Ok(());
            }
        }
    }
    pub async fn send_wait(
        message: &Command,
        address: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
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

            match decoded_reply {
                Ok(Reply::Done) => {
                    println!("Done!");
                    break;
                }
                Ok(Reply::List(l)) => {
                    for song in l.iter() {
                        println!("{song:?}");
                    }
                }
                _ => println!("reply unmanaged yet"),
            }
        }

        Ok(())
    }

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
#[derive(Display, Debug, Serialize, Deserialize, EnumString, EnumIter)]
pub enum Command {
    // "Music" commands
    Play(Option<String>),
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
