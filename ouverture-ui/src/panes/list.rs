use iced::Clipboard;
use iced::{
    button, pane_grid, pick_list, scrollable, Button, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Scrollable, Text,
};

use super::Content;
use crate::style;
use crate::Message;
use log::{debug, info};

use futures_util::pin_mut;
use ouverture_core::music::song::Song;

use futures_core::stream::Stream;
use futures_util::stream::StreamExt;
use ouverture_core::server::{self, Reply, Server};

pub struct List {
    list: Vec<Song>,
    scrollable: scrollable::State,

    theme: style::Theme,
}

impl Content for List {
    fn view(&mut self, _pane: pane_grid::Pane, _total_panes: usize) -> Element<Message> {
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
    }
    fn update(&mut self, message: Message, clipboard: &mut Clipboard) -> Command<Message> {
        debug!("in update for list");
        let server_address = "127.0.0.1:6603";
        let m = async {
            let r = Server::send(&server::Command::List(None), server_address).await;
            debug!("here");
            pin_mut!(r);
            debug!("here2");
            while let Some(reply) = r.next().await {
                debug!("{reply:?}");
            }
            Message::Nothing
        };

        // return pin_mut!(reply_stream);
        // let res = reply_stream.next();
        Command::from(m)
    }
}

impl List {
    pub fn new(theme: style::Theme) -> Self {
        List {
            list: vec![Song::default(), Song::default()],
            scrollable: scrollable::State::new(),
            theme,
        }
    }
}
