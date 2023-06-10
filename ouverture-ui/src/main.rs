use iced::{Application, Command, Element, Length, Settings, Subscription};

use iced::theme::Theme;
use iced::time;

use iced::widget::{container, pane_grid};

use iced_native::command::Action;
mod config;
mod opt;
mod style;
use config::Config;
use style::ThemeType;
pub mod panes;

use std::time::{Duration, Instant};

use ouverture_core::music::song::Song;

use panes::list;
use std::error::Error;

use opt::Opt;

use ouverture_core::logger::{setup_logger, LogDestination::*};

use structopt::StructOpt;

use log::LevelFilter::*;
use std::convert::Into;
use std::path::{Path, PathBuf};

use ouverture_core::server::{Command as ServerCommand, Reply, Server};

use futures_core::stream::Stream;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

use log::{debug, info, warn};
use panes::{Content, PaneMessage};

fn main() -> iced::Result {
    let opts = Opt::from_args();
    let level = match opts.log_level.as_deref() {
        None => Info,
        Some("trace") => Trace,
        Some("debug") => Debug,
        Some("info") => Info,
        Some("warn") => Warn,
        Some("error") => Error,
        Some("off") => Off,
        Some(_) => Info, // unreachable because of the arg parser
    };
    match opts.log_destination.clone() {
        None => setup_logger(StdErr, level),
        Some(path) => setup_logger(File(path), level),
    }
    .unwrap();
    debug!("Opts = {:?}", opts);

    Ouverture::run(Settings::with_flags(opts))
}

#[derive(Default)]
struct Ouverture {
    theme: Theme,
    panes: panes::Panes,
}

impl Ouverture {
    fn from_config(path: Option<PathBuf>) -> Self {
        let mut ouverture: Ouverture = Default::default();
        if let Some(config_path) = path {
            info!("Using custom config path: {config_path:?}");
            let read_config = Config::new(&config_path);
            if let Ok(c) = read_config {
                ouverture.theme = ThemeType::from(c.theme.into()).into();
            } else {
                warn!(
                    "Custom configuration is incomplete and couldn't be applied: {read_config:?}"
                );
            }
        }
        return ouverture;
    }
}

unsafe impl Send for Message {}
unsafe impl Sync for Message {}

#[derive(Debug, Clone)]
pub enum Message {
    Nothing,

    // SliderChanged(f32),
    ThemeChanged(ThemeType),

    //Pane Messages
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    Close(pane_grid::Pane),
    CloseFocused,

    ChildMessage(PaneMessage),

    //ControlBar messages
    Toggle,
    Play(Option<Song>),
    Pause,
    Next,
    Previous,

    //Menu messages
    Home,
    Library,
    Settings,

    // Editor messages
    IntoMenu(pane_grid::Pane),
    IntoControlBar(pane_grid::Pane),
    IntoSearchBar(pane_grid::Pane),
    IntoList(pane_grid::Pane),
    // // List message
}

impl From<PaneMessage> for Message {
    fn from(pm: PaneMessage) -> Message {
        Message::ChildMessage(pm)
    }
}

impl From<Message> for Action<Message> {
    fn from(val: Message) -> Self {
        Action::Future(Box::pin(async { val }))
    }
}

impl<'a> Application for Ouverture {
    type Message = Message;
    type Executor = iced_futures::backend::native::tokio::Executor;
    type Flags = Opt;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        (Ouverture::from_config(flags.config), Command::none())
    }

    fn title(&self) -> String {
        String::from("Ouverture")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        let address = "127.0.0.1:6603";
        debug!("top-level message: {:?}", message);
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme.into();
                Command::none()
            }

            Message::Play(opt_song) => Command::single(Action::Future(Box::pin(async move {
                let reply = Server::send_wait(&ServerCommand::Play(opt_song), address)
                    .await
                    .unwrap();
                debug!("GUI asked for play (status = {:?}", reply);
                Message::Nothing
            }))),
            Message::Toggle => Command::single(Action::Future(Box::pin(async move {
                debug!("GUI asking for toggle");
                let reply = Server::send_wait(&ServerCommand::Toggle, address)
                    .await
                    .unwrap();
                debug!("GUI asked for toggle, server replied: {:?}", reply);
                Message::Nothing
            }))),
            Message::Next => Command::single(Action::Future(Box::pin(async move {
                debug!("GUI asking for next");
                let reply = Server::send_wait(&ServerCommand::Next, address)
                    .await
                    .unwrap();
                debug!("GUI asked for next, server replied: {:?}", reply);
                Message::Nothing
            }))),
            Message::Previous => Command::single(Action::Future(Box::pin(async move {
                debug!("GUI asking for previous");
                let reply = Server::send_wait(&ServerCommand::Previous, address)
                    .await
                    .unwrap();
                debug!("GUI asked for previous, server replied: {:?}", reply);
                Message::Nothing
            }))),
            any => {
                debug!("updating panes");
                self.panes.update(any)
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
                time::every(Duration::from_millis(1000)).map(|i|Message::ChildMessage(PaneMessage::RefreshControl(i)))
    }

    fn view(&self) -> Element<Message> {
        let panes = self.panes.view();

        container(panes)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
