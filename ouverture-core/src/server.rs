use bincode;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::{Arc, Mutex};
use strum_macros::{Display, EnumIter, EnumString};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::config::Config;

pub struct Server {
    config: Config,
}

impl Server {
    pub async fn start() -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;

        let stop_flag = Arc::new(Mutex::new(false));

        loop {
            let local_stop_flag = stop_flag.clone();
            let (mut socket, _) = listener.accept().await?;

            tokio::spawn(async move {
                let mut buf = [0u8; 8];

                // In a loop, read all the data from the socket
                loop {
                    match socket.read(&mut buf).await {
                        // socket closed
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    let size = u64::from_ne_bytes(buf);

                    let mut payload = vec![0; size as usize];
                    let res = socket.read_exact(&mut payload[..]).await;
                    println!("res from socket = {:?}", res);

                    let decoded_command = bincode::deserialize::<Command>(&payload);
                    match decoded_command {
                        Ok(command) => match command {
                            Command::Play(_) => println!("Play"),
                            Command::Pause => println!("Pause"),
                            Command::Toggle => println!("Toggle"),
                            Command::Next => println!("Next"),
                            Command::Previous => println!("Previous"),
                            Command::Ping => println!("Ping"),
                            Command::Restart => println!("Restart"),
                            Command::Stop => {
                                let mut flag = local_stop_flag.lock().unwrap();
                                *flag = true
                            }

                            // Test commands
                            Command::Heavy(v) => println!("Heavy received, len = {}", v.len()),
                            #[allow(unreachable_patterns)]
                            _ => println!("Unknown command"),
                        },
                        Err(e) => eprint!("failed to decode message payload; err = {:?}", e),
                    };
                    println!("finished decoding comand");

                    // // Write the data back
                    // if let Err(e) = socket.write_all(&buf[0..n]).await {
                    //     eprintln!("failed to write to socket; err = {:?}", e);
                    //     return;
                    // }
                }
            });

            // in case the Stop command was received, exit the loop.
            // The binded address is released at 'listener' drop
            if *stop_flag.lock().unwrap() {
                break Ok(());
            }
        }
    }
    pub async fn send(message: &Command) -> Result<(), Box<dyn Error + Send + Sync>> {
        let encoded: Vec<u8> = message.prepare_query()?;
        let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
        stream.write_all(&encoded).await?;
        Ok(())
    }
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

    // "Server" commands
    Ping,
    Restart,
    Stop,

    // Test commands
    Heavy(Vec<u8>),
}

impl Command {
    fn prepare_query(&self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let content = self.to_string();
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
