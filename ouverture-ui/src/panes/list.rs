use iced::{
    alignment::Horizontal, button, pane_grid, scrollable, Alignment, Button, Column, Command,
    Container, Element, Length, Row, Scrollable, Text,
};
use log::debug;

use super::Content;
use crate::style;
use crate::style::stylesheet::*;
use crate::Message;

use iced_native::command::Action;
use std::rc::Rc;

use ouverture_core::music::song::Song;
use ouverture_core::server::Command as ServerCommand;
use ouverture_core::server::Server;

struct ListRow {
    selected: bool,
    title: String,
    artist: String,
}

impl From<Song> for ListRow {
    fn from(s: Song) -> Self {
        ListRow {
            selected: false,
            title: s.title.unwrap_or("Unknown".to_string()),
            artist: s.artist.unwrap_or("Unknown".to_string()),
        }
    }
}

pub struct List {
    content: Vec<ListRow>,
    scroll: scrollable::State,
    theme: style::Theme,
}

impl Content for List {
    fn view(&mut self, _pane: pane_grid::Pane, _total_panes: usize) -> Element<Message> {
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

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AskRefreshList(pane) => self.ask_refresh_list(pane),
            Message::ReceivedNewList(_pane, reply) => {
                self.got_refresh_list(&reply);
                Command::none()
            }
            _ => Command::none(),
        }
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

    pub fn ask_refresh_list(&mut self, pane: pane_grid::Pane) -> Command<Message> {
        let address = "127.0.0.1:6603";
        let cmd_ui = Command::single(Action::Future(Box::pin(async move {
            let reply = Server::send_wait(&ServerCommand::List(None), address)
                .await
                .unwrap();
            debug!("asked for list refresh");
            Message::ReceivedNewList(pane, Rc::new(reply))
        })));

        return cmd_ui;
    }

    pub fn got_refresh_list(&mut self, reply: &ouverture_core::server::Reply) {
        if let ouverture_core::server::Reply::List(vec_songs) = reply {
            debug!("got reply with new list");
            self.content = vec_songs.iter().map(|s| s.clone().into()).collect();
        }
    }
}
