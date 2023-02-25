use iced::{Application, Command, Element, Length, Settings};

use iced::theme::{Theme};

use iced::widget::{container, pane_grid};

use iced_native::command::Action;
mod opt;
pub mod panes;

use opt::Opt;

use ouverture_core::logger::{setup_logger, LogDestination::*};


use structopt::StructOpt;

use log::LevelFilter::*;

use std::rc::Rc;

use log::{debug};
// use panes::list::ColumnKey;
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
    // let config = match opts.config.clone() {
    //     None => Config::default(),
    //     Some(path) => Config::new_from_file(&path).unwrap(),
    // };

    Ouverture::run(Settings::with_flags(opts))
}

#[derive(Default)]
struct Ouverture {
    theme: Theme,
    panes: panes::Panes,
}

impl Ouverture {}

unsafe impl Send for Message {}
unsafe impl Sync for Message {}

#[derive(Debug, Clone)]
pub enum Message {
    Nothing,

    // SliderChanged(f32),
    ThemeChanged(Theme),

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
    AskRefreshList(pane_grid::Pane),
    ReceivedNewList(pane_grid::Pane, Rc<ouverture_core::server::Reply>),
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

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        // check for matching builtin themes
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Ouverture")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        debug!("top-level message: {:?}", message);
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
                Command::none()
            }
            any => {
                debug!("updating panes");
                self.panes.update(any)
            }
        }
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
}
