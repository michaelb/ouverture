use iced::widget::pane_grid::Configuration;
use iced::widget::{column, container, pane_grid, row, scrollable, text, Row};
use iced::{
    alignment::{Horizontal, Vertical},
    Command, Element, Length, Padding,
};
use iced_native::widget::button::{StyleSheet, Appearance};
use log::{debug, trace};
use std::string::ToString;
use strum::Display;

use super::Content;
use crate::panes::PaneMessage;
use crate::Message;

use iced_native::command::Action;
use std::rc::Rc;

use ouverture_core::music::song::Song;
use ouverture_core::server::Command as ServerCommand;
use ouverture_core::server::Server;

use iced::widget::button;
use iced::{Background,Vector, Color, Theme};

use crate::style::ThemeType;

struct ListRow {
    title: String,
    artist: String,
}

#[derive(Default)]
struct ListRowAppearance {

}

impl StyleSheet for ListRowAppearance {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {

        Appearance {
            .. Default::default()
        }
    }


}

impl From<Song> for ListRow {
    fn from(s: Song) -> Self {
        ListRow {
            title: s.title.unwrap_or("Unknown".to_string()),
            artist: s.artist.unwrap_or("Unknown".to_string()),
        }
    }
}

impl ListRow {
    fn view(&self) -> Element<Message> {
        let r = row![text(self.title.clone()), text(self.artist.clone())]
            .spacing(5)
            .width(150);
        return r.into();
    }
}

#[derive(Display, Debug, Copy, Clone)]
pub enum ColumnField {
    Title,
    Artist,
    Album,
}

pub struct List {
    rows: Vec<(Song, bool)>, // list of song and whether selected (used for coloring only)
    columns: Vec<(ColumnField, f32)>, // size and order of the columns
    current_sort: (ColumnField, bool), // current ordering (true = ascending, false) descending)
    current_selection: Option<usize>, // currently selected row
}

#[derive(Debug, Clone)]
pub enum ListMessage {
    ClickRow(Option<usize>),
    Sort(ColumnField),
}

impl Content for List {
    fn view(&self, _pane: pane_grid::Pane, _total_panes: usize) -> Element<Message> {
        let heading_button = |button_text: &str, sort_order: Option<ColumnField>| {
            let mut button =
                button(text(button_text).vertical_alignment(Vertical::Center)).padding(0);
            if let Some(order) = sort_order {
                button = button.on_press(Message::ChildMessage(PaneMessage::ListMessage(
                    ListMessage::Sort(order),
                )))
            }
            button
        };

        const ICON_COLUMN_WIDTH: f32 = 35.0;
        let header = container({
            let mut header_row = row![]
                .width(Length::Fill)
                .height(Length::Fixed(30.0))
                // Spacer heading for icons column
                .push(heading_button("X", None).width(Length::Fixed(ICON_COLUMN_WIDTH)));

            for c in self.columns.iter() {
                header_row = header_row
                    .push(heading_button(&c.0.to_string(), Some(c.0)).width(Length::Fixed(c.1)));
            }
            header_row
        })
        .padding(Padding::from([0, 8]))
        .width(Length::Fill);

        let mut rows = column![];
        for (i, s) in self.rows.iter().enumerate() {
            let e: ListRow = s.clone().0.into();
            let mut r = row![text("X").width(Length::Fixed(ICON_COLUMN_WIDTH))].spacing(5);
            for c in &self.columns {
                let field_content = match c.0 {
                    ColumnField::Title => e.title.clone(),
                    ColumnField::Artist => e.artist.clone(),
                    _ => todo!(),
                };
                r = r.push(text(field_content).width(Length::Fixed(c.1)));
            }
            let select_row_button = button(container(r).padding(Padding::from([0, 8])))
                .on_press(Message::ChildMessage(PaneMessage::ListMessage(
                    ListMessage::ClickRow(Some(i)),
                )))
                .height(Length::Fixed(30.0))
                .style(iced::theme::Button::Secondary)
                .padding(0);
            rows = rows.push(select_row_button);
        }

        let content = scrollable(rows);

        let header_and_list = column![header, content];

        container(header_and_list)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        if let Message::ChildMessage(cmessage) = message {
            return match cmessage {
                PaneMessage::AskRefreshList(pane) => self.ask_refresh_list(pane),
                PaneMessage::ReceivedNewList(_pane, reply) => {
                    self.got_refresh_list(&reply);
                    Command::none()
                }
                PaneMessage::ListMessage(ListMessage::ClickRow(Some(i))) => {
                    if !&self.rows[i].1 {
                        *(&mut self.rows[i].1) = true;
                        Command::none()
                    } else {
                        Command::single(Message::Play(Some(self.rows[i].0.clone())).into())
                    }
                }

                _ => Command::none(),
            };
        }
        Command::none()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl List {
    pub fn new() -> Self {
        List {
            rows: vec![],
            columns: vec![(ColumnField::Title, 600.0), (ColumnField::Artist, 350.0)],
            current_sort: (ColumnField::Title, true),
            current_selection: None,
        }
    }

    pub fn ask_refresh_list(&mut self, pane: pane_grid::Pane) -> Command<Message> {
        let address = "127.0.0.1:6603";

        Command::single(Action::Future(Box::pin(async move {
            let reply = Server::send_wait(&ServerCommand::GetList(None), address)
                .await
                .unwrap();
            debug!("asked for list refresh");
            PaneMessage::ReceivedNewList(pane, Rc::new(reply)).into()
        })))
    }

    pub fn got_refresh_list(&mut self, reply: &ouverture_core::server::Reply) {
        if let ouverture_core::server::Reply::List(vec_songs) = reply {
            debug!("got reply with new list");
            let songs: Vec<Song> = vec_songs.iter().map(|s| s.clone().into()).collect();

            self.rows = songs.into_iter().map(|s| (s, false)).collect();
        }
    }
}
