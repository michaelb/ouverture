use iced::{
    alignment::Horizontal, button, pane_grid, scrollable, Alignment, Button, Column, Container,
    Element, Length, Row, Scrollable, Text,
};

use super::Content;
use crate::style;
use crate::style::stylesheet::*;
use crate::Message;

struct ListRow {
    selected: bool,
    title: String,
    artist: String,
}

pub struct List {
    content: Vec<ListRow>,
    scroll: scrollable::State,
    theme: style::Theme,
}

impl Content for List {
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

impl List {
    pub fn new(theme: style::Theme) -> Self {
        List {
            content: vec![ListRow {
                selected: false,
                title: String::from("title"),
                artist: String::from("artist"),
            }],
            scroll: scrollable::State::new(),
            theme,
        }
    }

    fn view(&mut self) -> Element<Message> {
        let List {
            content,
            scroll,
            theme,
        } = self;
        let mut rows = Column::<Message>::new();
        for e in content {
            let r = Row::new()
                .spacing(5)
                .max_width(150)
                .push(Text::new(e.title.clone()))
                .push(Text::new(e.artist.clone()));
            rows = rows.push(r);
        }

        let content = Scrollable::new(scroll)
            .width(Length::Fill)
            .spacing(10)
            .push(rows)
            .align_items(Alignment::Start);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .into()
    }
}
