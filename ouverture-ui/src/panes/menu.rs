use iced::{button, pane_grid, Button, Container, Element, HorizontalAlignment, Length, Column, Text};

use super::Content;
use crate::style;
use crate::Message;

pub struct Menu {
    toggle: button::State,
    currently_playing: bool,
    next: button::State,
    previous: button::State,
    theme: style::Theme,
}

impl Content for Menu {
    fn view(&mut self, _pane: pane_grid::Pane, _total_panes: usize) -> Element<Message> {
        self.view()
    }
}

impl Menu {
    pub fn new(theme: style::Theme) -> Self {
        Menu {
            toggle: button::State::new(),
            currently_playing: false,
            next: button::State::new(),
            previous: button::State::new(),
            theme,
        }
    }

    fn view(&mut self) -> Element<Message> {
        let Menu {
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
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(16),
            )
            .width(Length::Fill)
            .padding(8)
            .on_press(message)
            .style(style)
        };

        let controls = Column::new()
            .spacing(5)
            .max_width(150)
            .push(button(previous, "Home", Message::Home, theme))
            .push(button(toggle, "Library", Message::Library, theme))
            .push(button(next, "Settings", Message::Settings, theme));

        Container::new(controls)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .into()
    }
}
