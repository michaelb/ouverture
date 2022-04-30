use super::header::*;
use crate::widgets::style::table_row;
use crate::widgets::{header, Header, TableRow};
use crate::Message;

use iced::Clipboard;
use iced::{
    button, pane_grid, pick_list, scrollable, Align, Button, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Scrollable, Text,
};
use ouverture_core::music::song::Song;

#[allow(clippy::too_many_arguments)]
pub fn data_row_container<'a, 'b>(
    // color_palette: ColorPalette,
    // config: &Config,
    song: &Song,
    column_config: &'b [(ColumnKey, Length, bool)],
    row_id: usize,
    // installed_for_flavor: bool,
    // install_addon: Option<&InstallAddon>,
) -> TableRow<'a, Message> {
    let default_height = Length::Units(26);
    let default_row_height = 26;

    let mut row_containers = vec![];

    // let addon_data = &addon.addon;
    // let install_button_state = &mut addon.install_button_state;
    //
    // let flavor_exists_for_addon = addon_data
    //     .versions
    //     .iter()
    //     .any(|gc| gc.flavor == config.wow.flavor.base_flavor());

    // if let Some((idx, width)) = column_config
    //     .iter()
    //     .enumerate()
    //     .filter_map(|(idx, (key, width, hidden))| {
    //         if *key == ColumnKey::Install && !hidden {
    //             Some((idx, width))
    //         } else {
    //             None
    //         }
    //     })
    //     .next()
    // {
    //     let status = install_addon.map(|a| a.status.clone());
    //
    //     let install_text = Text::new(match status {
    //         Some(InstallStatus::Downloading) => localized_string("downloading"),
    //         Some(InstallStatus::Unpacking) => localized_string("unpacking"),
    //         Some(InstallStatus::Retry) => localized_string("retry"),
    //         Some(InstallStatus::Unavailable) | Some(InstallStatus::Error(_)) => {
    //             localized_string("unavailable")
    //         }
    //         None => {
    //             if installed_for_flavor {
    //                 localized_string("installed")
    //             } else {
    //                 localized_string("install")
    //             }
    //         }
    //     })
    //     .size(16);

    // let install_wrapper = Container::new(install_text)
    //     .width(*width)
    //     .center_x()
    //     .align_x(Align::Center);
    //
    // let mut install_button = Button::new(install_button_state, install_wrapper)
    //     .style(style::SecondaryButton(color_palette))
    //     .width(*width);
    //
    // if flavor_exists_for_addon
    //     && (status == Some(InstallStatus::Retry) || (status == None && !installed_for_flavor))
    // {
    //     install_button = install_button.on_press(Interaction::InstallAddon(
    //         config.wow.flavor,
    //         addon_data.id.to_string(),
    //         InstallKind::Catalog {
    //             source: addon_data.source,
    //         },
    //     ));
    // }

    // let install_button: Element<Interaction> = install_button.into();
    //
    // let install_container = Container::new(install_button.map(Message::Interaction))
    //     .height(default_height)
    //     .width(*width)
    //     .center_y()
    //     .style(style::HoverableBrightForegroundContainer(color_palette));
    //
    // row_containers.push((idx, install_container));
    // }

    if let Some((idx, width)) =
        column_config
            .iter()
            .enumerate()
            .find_map(|(idx, (key, width, _))| {
                if *key == ColumnKey::Title {
                    Some((idx, width))
                } else {
                    None
                }
            })
    {
        let title = Text::new(song.title.clone().unwrap_or("?".to_string())).size(16);

        let title_container = Container::new(title)
            .padding(5)
            .height(default_height)
            .width(*width)
            .center_y();
        // .style(style::HoverableBrightForegroundContainer(color_palette));

        row_containers.push((idx, title_container));
    }
    if let Some((idx, width)) =
        column_config
            .iter()
            .enumerate()
            .find_map(|(idx, (key, width, _))| {
                if *key == ColumnKey::LocalVersion {
                    Some((idx, width))
                } else {
                    None
                }
            })
    {
        let title = Text::new(song.artist.clone().unwrap_or("?".to_string())).size(16);

        let title_container = Container::new(title)
            .padding(5)
            .height(default_height)
            .width(*width)
            .center_y();
        // .style(style::HoverableBrightForegroundContainer(color_palette));

        row_containers.push((idx, title_container));
    }

    // let left_spacer = Space::new(Length::Units(DEFAULT_PADDING), Length::Units(0));
    // let right_spacer = Space::new(Length::Units(DEFAULT_PADDING + 5), Length::Units(0));
    //
    // let mut row = Row::new().push(left_spacer).spacing(1);
    let mut row = Row::new().spacing(1);

    // Sort columns and push them into row
    row_containers.sort_by(|a, b| a.0.cmp(&b.0));
    for (_, elem) in row_containers.into_iter() {
        row = row.push(elem);
    }

    // row = row.push(right_spacer);

    let mut table_row = TableRow::new(row)
        .width(Length::Fill)
        .inner_row_height(default_row_height)
        .on_press(move |_| Message::RowCliked(row_id));

    if row_id % 2 == 0 {
        // table_row = table_row.style(style::TableRowAlternate(color_palette))
    } else {
        // table_row = table_row.style(style::TableRow(color_palette))
    }

    table_row
}
