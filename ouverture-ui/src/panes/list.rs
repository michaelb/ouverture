use iced::Clipboard;
use iced::{
    button, pane_grid, pick_list, scrollable, Button, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Scrollable, Text,
};
use std::any::Any;

use super::Content;
use crate::style;
use crate::Message;
use log::{debug, info};

use futures_util::pin_mut;
use ouverture_core::music::song::Song;

use crate::panes::PaneMessage;

use futures_core::stream::Stream;
use futures_util::stream::StreamExt;
use ouverture_core::server::{self, Reply, Server};
use std::sync::{Arc, Mutex};

pub struct List {
    list: Arc<Mutex<Vec<Song>>>,
    scrollable: scrollable::State,

    theme: style::Theme,
}

fn song_to_list_element<'a>(song: Song) -> Element<'a, Message> {
    let mut row = Row::new();
    row = row.push(Text::new(song.title.unwrap_or("?".to_string())));
    row = row.push(Text::new(song.artist.unwrap_or("?".to_string())));
    row.into()
}

impl Content for List {
    fn view(&mut self, _pane: pane_grid::Pane, _total_panes: usize) -> Element<Message> {
        let List {
            list,
            mut scrollable,
            mut theme,
        } = self;

        debug!("updating list view");
        debug!("self.list = {:?}", list);
        Container::new({
            let mut s = Scrollable::new(&mut self.scrollable).spacing(5);

            for song in list.lock().unwrap().iter() {
                s = s.push(song_to_list_element(song.clone()));
            }
            s
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(5)
        .into()
    }
    fn update(&mut self, message: Message, clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::ChildMessage(PaneMessage::Refresh(pane)) => {
                let server_address = "127.0.0.1:6603";
                let list = self.list.clone();
                let r = async move {
                    let r = Server::send(&server::Command::List(None), server_address).await;
                    pin_mut!(r);
                    while let Some(reply) = r.next().await {
                        if let Ok(reply) = reply {
                            match reply {
                                Reply::Received(s) => debug!("reply 'received': {s}"),
                                Reply::List(vec_song) => {
                                    debug!("received songs: {vec_song:?}");
                                    *list.lock().unwrap() = vec_song;
                                }
                                Reply::Done => break,
                                _ => debug!("unknown reply: {:?}", reply),
                            }
                        } else {
                            debug!("Err reply received: {:?}", reply);
                        }
                    }

                    Message::RefreshList(pane)
                };

                debug!("self.list = {:?}", self.list);

                Command::from(r)
            }
            _ => Command::none(),
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl List {
    pub fn new(theme: style::Theme) -> Self {
        List {
            list: Arc::new(Mutex::new(vec![])),
            scrollable: scrollable::State::new(),
            theme,
        }
    }
}
