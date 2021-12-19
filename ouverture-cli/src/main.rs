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

    ///Which ouverture server to communicate with
    #[structopt(long)]
    server: Option<String>,

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
    if let Some(optionnal_path) = opt.play.as_ref() {
        Server::send(&Command::Play(optionnal_path.clone())).await?;
    }
    if opt.pause {
        Server::send(&Command::Pause).await?;
    }
    if opt.toggle {
        Server::send(&Command::Toggle).await?;
    }
    if opt.next {
        Server::send(&Command::Next).await?;
    }
    if opt.previous {
        Server::send(&Command::Previous).await?;
    }
    if opt.ping {
        Server::send(&Command::Ping).await?;
    }

    Ok(())
}
