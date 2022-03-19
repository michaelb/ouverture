use iced::{
    button, pane_grid, pick_list, scrollable, Button, Column, Container, Element,
    HorizontalAlignment, Length, Row, Scrollable, Text,
};

use super::Content;
use crate::style;
use crate::Message;

use ouverture_core::music::song::Song;

pub struct List {
    list: Vec<Song>,
    scrollable: scrollable::State,

    theme: style::Theme,
}

impl Content for List {
    fn view(&mut self, _pane: pane_grid::Pane, _total_panes: usize) -> Element<Message> {
        self.view()
    }
}

impl List {
    pub fn new(theme: style::Theme) -> Self {
        List {
            list: vec![Song::default()],
            scrollable: scrollable::State::new(),
            theme,
        }
    }

    fn view(&mut self) -> Element<Message> {
        let List {
            list,
            mut scrollable,
            mut theme,
        } = self;

        let r = Row::new().push({
            let mut s = Scrollable::new(&mut self.scrollable).spacing(5);

            for song in list {
                s = s.push(Text::new(song.title.clone().unwrap_or("-".to_string())));
            }
            s
        });

        Container::new(r)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .into()

        // let button = |state, label, message, style| {
        //     Button::new(
        //         state,
        //         Text::new(label)
        //             .width(Length::Fill)
        //             .horizontal_alignment(HorizontalAlignment::Center)
        //             .size(16),
        //     )
        //     .width(Length::Fill)
        //     .padding(8)
        //     .on_press(message)
        //     .style(style)
        // };

        // let controls = Column::new()
        //     .spacing(5)
        //     .max_width(150)
        //     .push(button(previous, "Home", Message::Home, theme))
        //     .push(button(toggle, "Library", Message::Library, theme))
        //     .push(button(next, "Settings", Message::Settings, theme));
        //
        // Container::new(controls)
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .padding(5)
        //     .into()
    }
}
