use iced::widget::pane_grid::Configuration;
use iced::widget::{column, container, pane_grid, row, scrollable, text, Row};
use iced::{
    alignment::{Horizontal, Vertical},
    Command, Element, Length, Padding,
};
use iced_native::widget::button::{Appearance, StyleSheet};
use log::{debug, trace};
use std::string::ToString;
use strum::Display;

use super::Content;
use crate::Message;

use iced_runtime::command::Action;
use std::rc::Rc;

use crate::config::Config;
use ouverture_core::music::song::Song;
use ouverture_core::server::Command as ServerCommand;
use ouverture_core::server::Server;

use iced::widget::button;
use iced::{Background, Color, Theme, Vector};

use crate::style::ThemeType;

struct ListRow {
    title: String,
    artist: String,
}

#[derive(Default)]
struct ListRowAppearance {}

impl StyleSheet for ListRowAppearance {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        Appearance {
            ..Default::default()
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
        let r = row![text(self.title.clone()), text(self.artist.clone())];
        return r.into();
    }
}

#[derive(Display, Debug, Copy, Clone, PartialEq, Eq)]
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
    config: Config,
}

#[derive(Debug, Clone)]
pub enum ListMessage {
    ClickRow(Option<usize>),
    Sort(ColumnField),
}

impl Content for List {
    fn view(&self, _pane: pane_grid::Pane, _total_panes: usize) -> Element<Message> {
        let heading_button = |button_text: &str, sort_order: Option<ColumnField>| {
            let mut button = button(text(button_text).vertical_alignment(Vertical::Center));
            if let Some(order) = sort_order {
                button = button.on_press(Message::ListMessage(ListMessage::Sort(order)))
            }
            button
        };

        const ICON_COLUMN_WIDTH: f32 = 35.0;
        let header = container({
            let mut header_row = row![]
                .height(Length::Fixed(30.0))
                // Spacer heading for icons column
                .push(heading_button("X", None).width(Length::Fixed(ICON_COLUMN_WIDTH)));

            for c in self.columns.iter() {
                if c.0 == self.current_sort.0 {
                    let direction_marker = if self.current_sort.1 { "↑" } else { "↓" };
                    header_row = header_row.push(
                        heading_button(&(c.0.to_string() + " " + direction_marker), Some(c.0)).width(Length::Fixed(c.1)),
                    );
                } else {
                    header_row = header_row.push(
                        heading_button(&c.0.to_string(), Some(c.0)).width(Length::Fixed(c.1)),
                    );
                }
            }
            header_row
        })
        .width(Length::Fill);

        let mut rows = column![];
        for (i, s) in self.rows.iter().enumerate() {
            let e: ListRow = s.clone().0.into();
            let mut r = row![text("X").width(Length::Fixed(ICON_COLUMN_WIDTH))];
            for c in &self.columns {
                let field_content = match c.0 {
                    ColumnField::Title => e.title.clone(),
                    ColumnField::Artist => e.artist.clone(),
                    _ => todo!(),
                };
                r = r.push(text(field_content).width(Length::Fixed(c.1)));
            }
            let button_theme = if i % 2 == 0 {
                iced::theme::Button::Secondary
            } else {
                iced::theme::Button::Secondary // TODO custom theme for alternating colors in list
            };

            // TODO display in another color if is the selected song

            let select_row_button = button(r)
                .on_press(Message::ListMessage(ListMessage::ClickRow(Some(i))))
                .height(Length::Fixed(30.0))
                .style(button_theme);
            rows = rows.push(select_row_button);
        }

        let content = scrollable(rows);

        let header_and_list = column![header, content];

        container(header_and_list)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        return match message {
            Message::AskRefreshList(pane) => self.ask_refresh_list(pane),
            Message::ReceivedNewList(_pane, reply) => {
                self.got_refresh_list(&reply);
                Command::none()
            }
            Message::ListMessage(ListMessage::ClickRow(Some(i))) => {
                if !&self.rows[i].1 {
                    *(&mut self.rows[i].1) = true;
                    Command::none()
                } else {
                    Command::single(Message::Play(Some(self.rows[i].0.clone())).into())
                }
            }
            Message::ListMessage(ListMessage::Sort(field)) => {
                let mut order = self.current_sort.1;
                if self.current_sort.0 == field {
                    order = !order;
                }
                self.current_sort = (field, order);
                //
                // Sort in correct direction
                let sort_helper = |s: Song| match self.current_sort.0 {
                    ColumnField::Title => s.title.unwrap_or("".into()).to_lowercase(),
                    ColumnField::Artist => s.artist.unwrap_or("".into()).to_lowercase(),
                    ColumnField::Album => s.album.unwrap_or("".into()).to_lowercase(),
                };
                self.rows
                    .sort_by(|a, b| sort_helper(a.0.clone()).cmp(&sort_helper(b.0.clone())));
                // TODO more efficient sorting/inverting ?
                if !(order) {
                    self.rows.reverse();
                }
                Command::none()
            }

            _ => Command::none(),
        };
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl List {
    pub fn new(config: Config) -> Self {
        List {
            rows: vec![],
            columns: vec![(ColumnField::Title, 600.0), (ColumnField::Artist, 350.0)],
            current_sort: (ColumnField::Title, true),
            current_selection: None,
            config,
        }
    }

    pub fn ask_refresh_list(&mut self, pane: pane_grid::Pane) -> Command<Message> {
        let address =
            self.config.server_address.to_string() + ":" + &self.config.server_port.to_string();

        Command::single(Action::Future(Box::pin(async move {
            let reply = Server::send_wait(&ServerCommand::GetList(None), &address)
                .await
                .unwrap();
            debug!("asked for list refresh");
            Message::ReceivedNewList(pane, Rc::new(reply))
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
