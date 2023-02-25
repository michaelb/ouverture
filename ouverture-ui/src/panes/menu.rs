use iced::widget::{button, column, pane_grid, text, container};
use iced::{alignment::Horizontal, Element, Length};

use super::Content;
use crate::Message;

pub struct Menu {
    currently_playing: bool,
}

impl Content for Menu {
    fn view(&self, _pane: pane_grid::Pane, _total_panes: usize) -> Element<Message> {
        self.view()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            currently_playing: false,
        }
    }

    fn view(&self) -> Element<Message> {
        let Menu {
            currently_playing: _,
        } = self;

        let controls = column![
            button(text("Home")).on_press(Message::Home),
            button(text("Library")).on_press(Message::Library),
            button(text("Settings")).on_press(Message::Settings)
        ];

        container(controls)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .into()
    }
}
