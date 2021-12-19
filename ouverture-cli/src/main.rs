use color_eyre::eyre::eyre;
use color_eyre::{eyre::Report, eyre::WrapErr, Result, Section};
use ouverture_core::config::Config;
use ouverture_core::server::{Command, Server};
use std::error::Error;
use std::path::Path;
use structopt::StructOpt;

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

#[tokio::main(flavor = "multi_thread")]
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
    let mut command_count = 0;
    if opt.play.is_some() {
        command_count += 1
    }
    if opt.pause {
        command_count += 1
    }
    if opt.toggle {
        command_count += 1
    }
    if opt.next {
        command_count += 1
    }
    if opt.previous {
        command_count += 1
    }
    if command_count > 1 {
        return Err(Report::msg("More than one command provided!").suggestion(
            "Provide only one of --play, --pause, ,--toggle, --next or --previous as argument",
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
        Server::send(&Command::Stop, &server_addr).await?;
    }

    if let Some(optionnal_path) = opt.play.as_ref() {
        Server::send(&Command::Play(optionnal_path.clone()), &server_addr).await?;
    }
    if opt.pause {
        Server::send(&Command::Pause, &server_addr).await?;
    }
    if opt.toggle {
        Server::send(&Command::Toggle, &server_addr).await?;
    }
    if opt.next {
        Server::send(&Command::Next, &server_addr).await?;
    }
    if opt.previous {
        Server::send(&Command::Previous, &server_addr).await?;
    }
    if opt.ping {
        loop {
            let start = std::time::Instant::now();
            let status = Server::send(&Command::Ping, &server_addr).await;
            let duration = start.elapsed();
            match status {
                Ok(_) => println!(
                    "Server at {} reachable, time={}ms",
                    &server_addr,
                    duration.as_millis()
                ),
                Err(e) => println!("Could not reach server at {}: {:?}", &server_addr, e),
            }
            let sleep_for = std::time::Duration::from_secs(1);
            std::thread::sleep(sleep_for);
        }
    }

    Ok(())
}
