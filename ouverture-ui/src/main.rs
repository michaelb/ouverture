use iced::{Application, Command, Element, Length, Settings};

use iced::theme::Theme;

// use iced::Subscription;
// use iced::time;

use iced::widget::{container, pane_grid};

use iced_runtime::command::Action;
mod config;
mod opt;
mod style;
use config::Config;
use style::ThemeType;
pub mod panes;
use panes::Panes;

use std::rc::Rc;
use std::time::Instant;

use ouverture_core::music::song::Song;

use panes::list;

use opt::Opt;

use ouverture_core::logger::{setup_logger, LogDestination::*};
use ouverture_core::server::Command::Stop;

use structopt::StructOpt;

use log::LevelFilter::*;
use std::convert::Into;
use std::path::PathBuf;

use ouverture_core::server::{Command as ServerCommand, Server};

use log::{debug, error, info, warn};

use nix::unistd::fork;
use nix::unistd::ForkResult::{Child, Parent};

use daemonize::Daemonize;

fn main() -> iced::Result {
    let opts = Opt::from_args();
    debug!("Opts = {:?}", opts);
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

    let ui_config = match opts.config.clone() {
        None => Config::default(),
        Some(path) => {
            let c = Config::new(&path);
            c.unwrap_or_else(|_| {
                error!("Could not create config from the provided file {:?}", &path);
                Config::default()
            })
        }
    };

    if !ui_config.external_server {
        let pid = fork();
        match pid.expect("Fork Failed: Unable to create child process!") {
            Child => {
                let server_config = match opts.config {
                    None => ouverture_core::config::Config::default(),
                    Some(path) => {
                        let c = ouverture_core::config::Config::new_from_file(&path);
                        c.unwrap_or_else(|_| {
                            error!("Could not create config from the provided file {:?}", &path);
                            ouverture_core::config::Config::default()
                        })
                    }
                };

                if server_config.background {
                    let daemonize = Daemonize::new();
                    match daemonize.start() {
                        Ok(_) => {
                            info!("Successfully forked ouverture-server process to the background")
                        }
                        Err(_) => error!("Failed to daemonize ouverture-server"),
                    }
                }
                let res = ouverture_core::start_with_handlers(server_config);
                info!("ouverture server exited: {:?}", res);
                return Ok(());
            }
            Parent { child: _ } => info!("forked ouverture into server and UI processes"),
        }
    }
    let s = Settings::with_flags(opts);

    let res = Ouverture::run(s);

    // when this finishes (may be due to a graphical kill)
    // send a 'stop' command to the server if not external and not background
    if (!ui_config.external_server) && (!ui_config.background) {
        let address = ui_config.server_address + ":" + &ui_config.server_port.to_string();
        let server_stop_res = Server::send_wait_sync(&Stop, &address);
        match server_stop_res {
            Ok(_) => info!("Server stopped gracefully"),
            Err(e) => warn!("Failed to stop ouverture server at address {address}: {e:?}"),
        }
    }
    return res;
}

struct Ouverture {
    panes: panes::Panes,
    config: Config,
}

impl Ouverture {
    fn from_config(path: Option<PathBuf>) -> Self {
        if let Some(config_path) = path {
            info!("Using custom config path: {config_path:?}");
            let read_config = Config::new(&config_path);
            if let Ok(c) = read_config {
                return Ouverture {
                    panes: Panes::new(&c),
                    config: c,
                };
            } else {
                warn!(
                    "Custom configuration is incomplete and couldn't be applied: {read_config:?}"
                );
            }
        }
        return Ouverture {
            panes: Panes::new(&Config::default()),
            config: Config::default(),
        };
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

    //ControlBar messages
    Toggle,
    Play(Option<Song>),
    Pause,
    Next,
    Previous,
    SliderChanged(u32),     // changed by user, seek song to new position
    SliderChangedAuto(u32), // updated by server, don't seek new position
    RefreshControl(Instant),

    //Menu messages
    Home,
    Library,
    Settings,

    // Editor messages
    IntoMenu(pane_grid::Pane),
    IntoControlBar(pane_grid::Pane),
    IntoSearchBar(pane_grid::Pane),
    IntoList(pane_grid::Pane),

    // List message
    AskRefreshList(pane_grid::Pane),
    ReceivedNewList(pane_grid::Pane, Rc<ouverture_core::server::Reply>),
    ListMessage(list::ListMessage),
    ReceivedNewCurrentSong(Option<Song>, f32),

    // Misc
    ServerReply(pane_grid::Pane),
    Refresh(pane_grid::Pane),
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
        let address =
            self.config.server_address.to_string() + ":" + &self.config.server_port.to_string();
        debug!("top-level message: {:?}", message);
        match message {
            Message::ThemeChanged(theme) => {
                self.config.theme = theme.into();
                Command::none()
            }

            Message::Play(opt_song) => Command::single(Action::Future(Box::pin(async move {
                let reply = Server::send_wait(&ServerCommand::Play(opt_song), &address)
                    .await
                    .unwrap();
                debug!("GUI asked for play (status = {:?}", reply);
                Message::Nothing
            }))),
            Message::Toggle => Command::single(Action::Future(Box::pin(async move {
                debug!("GUI asking for toggle");
                let reply = Server::send_wait(&ServerCommand::Toggle, &address)
                    .await
                    .unwrap();
                debug!("GUI asked for toggle, server replied: {:?}", reply);
                Message::Nothing
            }))),
            Message::Next => Command::single(Action::Future(Box::pin(async move {
                debug!("GUI asking for next");
                let reply = Server::send_wait(&ServerCommand::Next, &address)
                    .await
                    .unwrap();
                debug!("GUI asked for next, server replied: {:?}", reply);
                Message::Nothing
            }))),
            Message::Previous => Command::single(Action::Future(Box::pin(async move {
                debug!("GUI asking for previous");
                let reply = Server::send_wait(&ServerCommand::Previous, &address)
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

    // fn subscription(&self) -> Subscription<Message> {
    // TODO reactivate poll or do thing in another way to reflect server changes
    // time::every(Duration::from_millis(1000)).map(|i| Message::RefreshControl(i))
    // }

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
        self.config.get_theme()
    }
}
