use iced::{
    executor, pane_grid, Align, Application, Clipboard, Column, Command, Container, Element,
    Length, Row, Settings, Text,
};
pub mod panes;
pub mod style;

use panes::Content;

#[tokio::main]
async fn main() -> iced::Result {
    Ouverture::run(Settings::default())
}

#[derive(Default)]
struct Ouverture {
    theme: style::Theme,
    panes: panes::Panes,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
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
}

impl Application for Ouverture {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Ouverture")
    }

    fn update(&mut self, message: Message, clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::ThemeChanged(theme) => self.theme = theme,
            any => {
                self.panes.update(any, clipboard);
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let panes = self.panes.view();

        Container::new(panes)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
    }
}
