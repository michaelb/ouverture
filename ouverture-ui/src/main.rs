use iced::{
    executor, pane_grid, Alignment, Application, Column, Command, Container, Element, Length, Row,
    Settings, Text,
};
mod opt;
pub mod panes;
pub mod style;

use opt::Opt;
use ouverture_core::config::Config;
use ouverture_core::logger::{setup_logger, LogDestination::*};
use ouverture_core::music::song::Song;
use ouverture_core::start;
use structopt::StructOpt;

use log::LevelFilter::*;
use style::stylesheet::*;

use log::{debug, info, warn};
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
    theme: style::Theme,
    panes: panes::Panes,
}

impl Ouverture {
    fn with_theme(theme: style::Theme) -> Self {
        Ouverture {
            theme,
            panes: panes::Panes::with_theme(theme),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Nothing,

    // SliderChanged(f32),
    ThemeChanged(style::Theme),

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
    // Scrolled(f32),
}

impl<'a> Application for Ouverture {
    type Message = Message;
    type Executor = iced_futures::backend::native::tokio::Executor;
    type Flags = Opt;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        // check for matching builtin themes
        if let Some(theme_str) = _flags.theme {
            for builtin_theme in style::Theme::all() {
                if theme_str.to_lowercase() == builtin_theme.0 {
                    return (Self::with_theme(builtin_theme.1), Command::none());
                }
            }
        }

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

    fn view(&mut self) -> Element<Message> {
        let panes = self.panes.view();

        Container::new(panes)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(NormalBackgroundContainer(self.theme))
            .into()
    }
}
