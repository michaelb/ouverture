use iced::{
    alignment::Horizontal, Element, Length};

use iced::widget::{ button, pane_grid, text , row, container};

use super::Content;
use crate::Message;

pub struct ControlBar {
    currently_playing: bool,
}

impl Content for ControlBar {
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

impl ControlBar {
    pub fn new() -> Self {
        ControlBar {
            currently_playing: false,
        }
    }

    fn view(&self) -> Element<Message> {
        let ControlBar {
            currently_playing: _,
        } = self;

        let controls = row![]
            .spacing(5)
            .push(button( text("<-")).on_press( Message::Previous))
            .push(button( text(">")).on_press( Message::Toggle))
            .push(button( text("->")).on_press( Message::Next));

        container(controls)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .into()
    }
}
