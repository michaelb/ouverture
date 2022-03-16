use color_eyre::eyre::eyre;
use color_eyre::{eyre::Report, eyre::WrapErr, Result, Section};
use ouverture_core::config::Config;
use ouverture_core::server::{Command, Reply, Server};
use std::error::Error;
use std::path::Path;
use std::time::Duration;
use structopt::StructOpt;
use tokio::time::timeout;

use futures_core::stream::Stream;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ouverture-cli",
    about = "The command-line interface to the ouverture music player"
)]
struct Opt {
    /// Resume playing the current song, or play the specified file
    #[structopt(long)]
    play: Option<Option<String>>,

    /// Pause the current song
    #[structopt(long)]
    pause: bool,

    /// Toggle play/pause
    #[structopt(long)]
    toggle: bool,

    ///Play the next song
    #[structopt(long)]
    next: bool,

    ///Play the previous song
    #[structopt(long)]
    previous: bool,

    /// Scan the library
    #[structopt(long)]
    scan: bool,

    /// List the songs (matching an optionnal criteria)
    #[structopt(long)]
    list: Option<Option<String>>,

    ///Stop the server
    #[structopt(long)]
    stop: bool,

    ///Which ouverture server to communicate with
    #[structopt(long)]
    server: Option<String>,

    /// Ouverture server port (default to 6603)
    #[structopt(long)]
    port: Option<String>,

    ///Ping the server
    #[structopt(long)]
    ping: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    check_unique_command(&opt)?;

    match launch_command(&opt).await {
        Ok(_) => Ok(()),
        Err(e) => Err(eyre!(e)),
    }
}

/// Checks if only one argument was given as argument
/// Otherwise, the user won't probably understand what's happening
fn check_unique_command(opt: &Opt) -> Result<()> {
    let command_count = [
        opt.play.is_some(),
        opt.pause,
        opt.toggle,
        opt.next,
        opt.previous,
        opt.ping,
        opt.list.is_some(),
        opt.scan,
    ]
    .into_iter()
    .filter(|b| *b)
    .count();
    if command_count > 1 {
        return Err(Report::msg("More than one command provided!").suggestion(
            "Provide only one of --play, --pause, --ping, --scan, --list ,--toggle, --next or --previous as argument",
        ));
    }
    Ok(())
}

async fn launch_command(opt: &Opt) -> Result<(), Box<dyn Error + Send + Sync>> {
    let server_addr = opt
        .server
        .as_ref()
        .unwrap_or(&"127.0.0.1".to_string())
        .to_string()
        + ":"
        + &opt.port.as_ref().unwrap_or(&String::from("6603"));

    if opt.stop {
        Server::send_wait(&Command::Stop, &server_addr).await?;
    }

    if let Some(optionnal_path) = opt.play.as_ref() {
        Server::send_wait(&Command::Play(optionnal_path.clone()), &server_addr).await?;
    }
    if opt.pause {
        Server::send_wait(&Command::Pause, &server_addr).await?;
    }
    if opt.toggle {
        Server::send_wait(&Command::Toggle, &server_addr).await?;
    }
    if opt.next {
        Server::send_wait(&Command::Next, &server_addr).await?;
    }
    if opt.previous {
        Server::send_wait(&Command::Previous, &server_addr).await?;
    }
    if opt.scan {
        Server::send_wait(&Command::Scan, &server_addr).await?;
    }
    if let Some(optionnal_str) = opt.list.as_ref() {
        handle(Server::send(&Command::List(optionnal_str.clone()), &server_addr).await).await;
    }

    if opt.ping {
        loop {
            let start = std::time::Instant::now();
            let status_with_timeout = timeout(
                Duration::from_secs(1),
                Server::send_wait(&Command::Ping, &server_addr),
            )
            .await;
            let duration = start.elapsed();
            match status_with_timeout {
                Ok(status) => match status {
                    Ok(_) => println!(
                        "Server at {} reachable, time={}ms",
                        &server_addr,
                        duration.as_millis()
                    ),
                    Err(e) => println!("Could not reach server at {}: {:?}", &server_addr, e),
                },
                Err(_) => println!("Timeout trying to reach server at {}", &server_addr),
            }
            let sleep_for = std::time::Duration::from_secs(1);
            std::thread::sleep(sleep_for);
        }
    }

    Ok(())
}

async fn handle<T>(stream: T)
where
    T: Stream<Item = Result<Reply, Box<dyn Error + Send + Sync>>>,
{
    pin_mut!(stream);
    while let Some(reply) = stream.next().await {
        match reply {
            Ok(Reply::Done) => println!("Done!"),
            Ok(Reply::List(l)) => println!("Result: {:?}", l),
            Err(e) => println!("Error: {:?}", e),
            _ => println!("unamagned reply yet"),
        }
    }
}
