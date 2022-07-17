use iced::{
    alignment::Horizontal, button, pane_grid, pick_list, scrollable, Alignment, Button, Column,
    Command, Container, Element, Length, Row, Scrollable, Text,
};
use std::any::Any;

use super::Content;
use crate::style;
use crate::Message;
use log::{debug, info};
mod header;
mod table_row;
pub use header::*;
pub use table_row::*;

use futures_util::pin_mut;
use ouverture_core::music::song::Song;

use crate::panes::PaneMessage;

use futures_core::stream::Stream;
use futures_util::stream::StreamExt;
use ouverture_core::server::{self, Reply, Server};
use std::sync::{Arc, Mutex};

use crate::widgets::header::ResizeEvent;

pub struct List {
    list: Arc<Mutex<Vec<Song>>>,
    scrollable: scrollable::State,
    theme: style::Theme,
    header: HeaderState,
}

impl Content for List {
    fn view(&mut self, pane: pane_grid::Pane, _total_panes: usize) -> Element<Message> {
        let List {
            list,
            mut scrollable,
            mut theme,
            header,
        } = self;

        debug!("updating list view");
        debug!("self.list = {:?}", list);
        let column_config: Vec<_> = header
            .columns
            .iter()
            .map(|h| (h.key, h.width, h.hidden))
            .collect();
        let mut global_column = Column::new();
        global_column = global_column.push(titles_row_header(
            &mut self.header.state,
            &mut self.header.columns,
            pane,
            None,
        ));
        for (i, song) in self.list.lock().unwrap().iter().enumerate() {
            global_column = global_column.push(data_row_container(song, &column_config, i));
        }
        Container::new({ global_column })
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .into()
    }
    fn update(&mut self, message: Message) -> Command<Message> {
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
            Message::ResizeColumn(pane, event) => {
                if let ResizeEvent::ResizeColumn {
                    left_key,
                    left_width,
                    right_key,
                    right_width,
                } = event
                {
                    if let Some(column) = self
                        .header
                        .columns
                        .iter_mut()
                        .find(|c| c.key as usize == left_key)
                    {
                        column.width = Length::Units(left_width);
                    }

                    if let Some(column) = self
                        .header
                        .columns
                        .iter_mut()
                        .find(|c| c.key as usize == right_key)
                    {
                        column.width = Length::Units(right_width);
                    }
                }
                Command::none()
            }
            x => {
                debug!("discarded message: {:?}", x);
                Command::none()
            }
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
            header: Default::default(),
        }
    }
}
