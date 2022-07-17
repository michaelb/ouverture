use iced::{
    alignment::Horizontal, button, pane_grid, Button, Container, Element, Length, Row, Text,
};

use super::Content;
use crate::style;
use crate::style::stylesheet::*;
use crate::Message;

pub struct ControlBar {
    toggle: button::State,
    currently_playing: bool,
    next: button::State,
    previous: button::State,
    theme: style::Theme,
}

impl Content for ControlBar {
    fn view(&mut self, _pane: pane_grid::Pane, _total_panes: usize) -> Element<Message> {
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
    pub fn new(theme: style::Theme) -> Self {
        ControlBar {
            toggle: button::State::new(),
            currently_playing: false,
            next: button::State::new(),
            previous: button::State::new(),
            theme,
        }
    }

    fn view(&mut self) -> Element<Message> {
        let ControlBar {
            toggle,
            currently_playing: _,
            next,
            previous,
            mut theme,
        } = self;

        let button = |state, label, message, style| {
            Button::new(
                state,
                Text::new(label)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center)
                    .size(16),
            )
            .width(Length::Fill)
            .padding(8)
            .on_press(message)
            .style(NormalTextButton(style))
        };

        let controls = Row::new()
            .spacing(5)
            .max_width(150)
            .push(button(previous, "<-", Message::Previous, theme))
            .push(button(toggle, ">", Message::Toggle, theme))
            .push(button(next, "->", Message::Next, theme));

        Container::new(controls)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .into()
    }
}
