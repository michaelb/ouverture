use crate::widgets::{header, Header, TableRow};
use crate::Message;

use iced::Clipboard;
use iced::{
    button, pane_grid, pick_list, scrollable, Align, Button, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Scrollable, Text,
};

#[derive(Copy, Clone)]
pub enum SortDirection {
    Asc,
    Desc,
}

pub fn localized_string(str: &str) -> String {
    str.to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum ColumnKey {
    Title = 0,
    LocalVersion,
    RemoteVersion,
}
#[derive(Debug, Clone)]
pub enum Flavor {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstallKind {
    Source,
}

#[derive(Debug, Clone)]
pub enum ExpandType {
    Changelog { changelog: Option<String> },
    None,
}

fn row_title<T: PartialEq>(
    column_key: T,
    previous_column_key: Option<T>,
    previous_sort_direction: Option<SortDirection>,
    title: &str,
) -> String {
    if Some(column_key) == previous_column_key {
        match previous_sort_direction {
            Some(SortDirection::Asc) => format!("{} ▲", title),
            Some(SortDirection::Desc) => format!("{} ▼", title),
            _ => title.to_string(),
        }
    } else {
        title.to_string()
    }
}

impl From<&str> for ColumnKey {
    fn from(s: &str) -> Self {
        match s {
            "title" => ColumnKey::Title,
            "local" => ColumnKey::LocalVersion,
            "remote" => ColumnKey::RemoteVersion,
            _ => panic!("Unknown ColumnKey for {}", s),
        }
    }
}

impl ColumnKey {
    fn title(self) -> String {
        use ColumnKey::*;

        match self {
            Title => localized_string("title"),
            LocalVersion => localized_string("localver"),
            RemoteVersion => localized_string("remote ver"),
        }
    }

    fn as_string(self) -> String {
        use ColumnKey::*;

        let s = match self {
            Title => "title",
            LocalVersion => "local",
            RemoteVersion => "remote",
        };

        s.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ColumnState {
    pub key: ColumnKey,
    pub btn_state: button::State,
    pub width: Length,
    pub hidden: bool,
    pub order: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Catalog,
    Install,
    Settings,
    About,
}

pub struct HeaderState {
    pub state: header::State,
    pub previous_column_key: Option<ColumnKey>,
    pub previous_sort_direction: Option<SortDirection>,
    pub columns: Vec<ColumnState>,
}

pub fn titles_row_header<'a>(
    // color_palette: ColorPalette,
    // catalog: &Catalog,
    header_state: &'a mut header::State,
    column_state: &'a mut [ColumnState],
    pane: pane_grid::Pane,
    // previous_column_key: Option<CatalogColumnKey>,
    previous_sort_direction: Option<SortDirection>,
) -> Header<'a, Message> {
    // A row containing titles above the addon rows.
    let mut row_titles = vec![];

    for column in column_state.iter_mut().filter(|c| !c.hidden) {
        let column_key = column.key;

        let row_title = row_title(
            column_key,
            None,
            previous_sort_direction,
            &column.key.title(),
        );

        let mut row_header = Button::new(
            &mut column.btn_state,
            Text::new(row_title).width(Length::Fill),
        )
        .width(Length::Fill);

        // if column_key != CatalogColumnKey::Install {
        //     row_header = row_header.on_press(Interaction::SortCatalogColumn(column_key));
        // }

        // if previous_column_key == Some(column_key) {
        //     row_header = row_header.style(style::SelectedColumnHeaderButton(color_palette));
        // } else if column_key == CatalogColumnKey::Install {
        //     row_header = row_header.style(style::UnclickableColumnHeaderButton(color_palette));
        // } else {
        //     row_header = row_header.style(style::ColumnHeaderButton(color_palette));
        // }

        let row_header: Element<Message> = row_header.into();

        let row_container = Container::new(row_header).width(column.width);

        // // Only shows row titles if we have any catalog results.
        // if !catalog.addons.is_empty() {
        row_titles.push((column.key as usize, row_container));
        // }
    }

    Header::new(
        header_state,
        row_titles,
        Some(Length::Units(10)),
        Some(Length::Units(10 + 5)),
    )
    .spacing(1)
    .height(Length::Units(25))
    .on_resize(3, move |event| Message::ResizeColumn(pane, event))
}

impl Default for HeaderState {
    fn default() -> Self {
        Self {
            state: Default::default(),
            previous_column_key: None,
            previous_sort_direction: None,
            columns: vec![
                ColumnState {
                    key: ColumnKey::Title,
                    btn_state: Default::default(),
                    width: Length::Fill,
                    hidden: false,
                    order: 0,
                },
                ColumnState {
                    key: ColumnKey::LocalVersion,
                    btn_state: Default::default(),
                    width: Length::Fill,
                    hidden: false,
                    order: 0,
                },
                ColumnState {
                    key: ColumnKey::RemoteVersion,
                    btn_state: Default::default(),
                    width: Length::Fill,
                    hidden: false,
                    order: 0,
                },
            ],
        }
    }
}
