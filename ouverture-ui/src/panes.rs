use futures_core::stream::Stream;
use iced::{
    alignment::Horizontal, button, executor, keyboard, pane_grid, scrollable, Alignment,
    Application, Button, Column, Command, Container, Element, Length, PaneGrid, Row, Scrollable,
    Text,
};
use std::sync::Arc;

pub struct Panes {
    panes: pane_grid::State<Box<dyn Content>>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
    theme: style::Theme,
}
use crate::style;
use crate::style::stylesheet::*;
use crate::Message;
use log::{debug, trace, warn};
use ouverture_core::server::Reply;
use std::any::Any;

mod control_bar;
// pub mod list;
mod list;
mod menu;

#[derive(Debug, Clone, Copy)]
pub enum PaneMessage {
    ServerReply(pane_grid::Pane),
    Refresh(pane_grid::Pane),
}

impl Panes {
    pub fn with_theme(theme: style::Theme) -> Self {
        let a: Box<dyn Content> = Box::new(Editor::new(0, style::Theme::default()));
        let b: Box<dyn Content> = Box::new(control_bar::ControlBar::new(style::Theme::default()));

        let a_conf = Box::new(iced::widget::pane_grid::Configuration::Pane(a));
        let b_conf = Box::new(iced::widget::pane_grid::Configuration::Pane(b));

        let c = iced::widget::pane_grid::Configuration::Split {
            axis: iced::widget::pane_grid::Axis::Vertical,
            ratio: 0.75,
            a: a_conf,
            b: b_conf,
        };

        let panes = pane_grid::State::with_configuration(c);

        Panes {
            panes,
            panes_created: 1,
            focus: None,
            theme,
        }
    }
}

impl Default for Panes {
    fn default() -> Self {
        Self::with_theme(style::Theme::default())
    }
}

pub struct PaneFlags {
    theme: style::Theme,
}

impl Application for Panes {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = PaneFlags;

    fn new(flags: PaneFlags) -> (Self, Command<Message>) {
        (Panes::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Pane grid - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        use Message::*;
        match message {
            Split(axis, pane) => {
                let result = self.panes.split(
                    axis,
                    &pane,
                    Box::new(Editor::new(self.panes_created, self.theme)),
                );

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }

                self.panes_created += 1;
            }
            SplitFocused(axis) => {
                if let Some(pane) = self.focus {
                    let result = self.panes.split(
                        axis,
                        &pane,
                        Box::new(Editor::new(self.panes_created, self.theme)),
                    );

                    if let Some((pane, _)) = result {
                        self.focus = Some(pane);
                    }

                    self.panes_created += 1;
                }
            }
            FocusAdjacent(direction) => {
                if let Some(pane) = self.focus {
                    if let Some(adjacent) = self.panes.adjacent(&pane, direction) {
                        self.focus = Some(adjacent);
                    }
                }
            }
            Clicked(pane) => {
                self.focus = Some(pane);
            }
            Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(&split, ratio);
            }
            Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.swap(&pane, &target);
            }
            Dragged(_) => {}
            Close(pane) => {
                if let Some((_, sibling)) = self.panes.close(&pane) {
                    self.focus = Some(sibling);
                }
            }
            CloseFocused => {
                if let Some(pane) = self.focus {
                    if let Some((_, sibling)) = self.panes.close(&pane) {
                        self.focus = Some(sibling);
                    }
                }
            }
            IntoMenu(pane) => {
                let menu = menu::Menu::new(self.theme);
                let result = self
                    .panes
                    .split(pane_grid::Axis::Horizontal, &pane, Box::new(menu));

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }
                self.panes.close(&pane);
            }
            IntoList(pane) => {
                let list = list::List::new(self.theme);
                let result = self
                    .panes
                    .split(pane_grid::Axis::Horizontal, &pane, Box::new(list));

                if let Some((new_pane, _)) = result {
                    self.focus = Some(new_pane);
                    self.panes.close(&pane);
                    return Command::single(AskRefreshList(new_pane).into());
                } else {
                    warn!("failed to close pane, keeping current one");
                };
            }

            IntoControlBar(pane) => {
                let menu = control_bar::ControlBar::new(self.theme);
                let result = self
                    .panes
                    .split(pane_grid::Axis::Horizontal, &pane, Box::new(menu));

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }
                self.panes.close(&pane);
            }
            AskRefreshList(pane) => {
                let mut list: &mut list::List = self
                    .panes
                    .get_mut(&pane)
                    .unwrap()
                    .as_any_mut()
                    .downcast_mut::<list::List>()
                    .unwrap();
                return list.update(AskRefreshList(pane));
            }
            ReceivedNewList(pane, reply) => {
                let mut list: &mut list::List = self
                    .panes
                    .get_mut(&pane)
                    .unwrap()
                    .as_any_mut()
                    .downcast_mut::<list::List>()
                    .unwrap();
                return list.update(ReceivedNewList(pane, reply));
            }
            // ResizeColumn(pane, event) => {
            //     let mut list: &mut list::List = self
            //         .panes
            //         .get_mut(&pane)
            //         .unwrap()
            //         .as_any_mut()
            //         .downcast_mut::<list::List>()
            //         .unwrap();
            //     return list.update(ResizeColumn(pane, event));
            // }
            ChildMessage(msg) => {
                let command = Command::batch(
                    self.panes
                        .iter_mut()
                        .map(|(p, s)| s.update(ChildMessage(msg))),
                );
                debug!("passing message to children");
                return command;
            }
            _ => (),
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let focus = self.focus;
        let total_panes = self.panes.len();

        let theme = self.theme;
        PaneGrid::new(&mut self.panes, |pane, content| {
            let is_focused = focus == Some(pane);

            let title_bar: pane_grid::TitleBar<Message> = if is_focused {
                pane_grid::TitleBar::new(Text::new("focused")).padding(10)
            } else {
                pane_grid::TitleBar::new(Text::new("not focused")).padding(10)
            };
            debug!("updating view in panes");
            pane_grid::Content::new(content.view(pane, total_panes))
                .title_bar(title_bar)
                .style(NormalBackgroundContainer(theme))
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
        .on_click(Message::Clicked)
        .on_drag(Message::Dragged)
        .on_resize(10, Message::Resized)
        .into()
    }
}

fn handle_hotkey(key_code: keyboard::KeyCode) -> Option<Message> {
    use keyboard::KeyCode;
    use pane_grid::{Axis, Direction};

    let direction = match key_code {
        KeyCode::Up => Some(Direction::Up),
        KeyCode::Down => Some(Direction::Down),
        KeyCode::Left => Some(Direction::Left),
        KeyCode::Right => Some(Direction::Right),
        _ => None,
    };

    match key_code {
        KeyCode::V => Some(Message::SplitFocused(Axis::Vertical)),
        KeyCode::H => Some(Message::SplitFocused(Axis::Horizontal)),
        KeyCode::W => Some(Message::CloseFocused),
        _ => direction.map(Message::FocusAdjacent),
    }
}

pub trait Content {
    fn view(&mut self, pane: pane_grid::Pane, total_panes: usize) -> Element<Message>;
    fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

struct Editor {
    id: usize,
    scroll: scrollable::State,
    split_horizontally: button::State,
    split_vertically: button::State,
    into_menu: button::State,
    into_controlbar: button::State,
    into_list: button::State,
    close: button::State,
    theme: style::Theme,
}

impl Editor {
    fn new(id: usize, theme: style::Theme) -> Self {
        Editor {
            id,
            scroll: scrollable::State::new(),
            split_horizontally: button::State::new(),
            split_vertically: button::State::new(),
            into_menu: button::State::new(),
            into_controlbar: button::State::new(),
            into_list: button::State::new(),
            close: button::State::new(),
            theme,
        }
    }
}
impl Content for Editor {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn view(&mut self, pane: pane_grid::Pane, total_panes: usize) -> Element<Message> {
        let Editor {
            scroll,
            split_horizontally,
            split_vertically,
            into_menu,
            into_controlbar,
            into_list,
            close,
            ..
        } = self;

        let button = |state, label, message, style| {
            Button::new(
                state,
                Text::new(label)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center)
                    .size(16),
            )
            .width(Length::Fill)
            .padding(8)
            .on_press(message)
            .style(NormalTextButton(style))
        };

        let mut controls = Column::new()
            .spacing(5)
            .max_width(150)
            .push(button(
                split_horizontally,
                "-",
                Message::Split(pane_grid::Axis::Horizontal, pane),
                self.theme,
            ))
            .push(button(
                into_menu,
                "Menu",
                Message::IntoMenu(pane),
                self.theme,
            ))
            .push(button(
                into_controlbar,
                "ControlBar",
                Message::IntoControlBar(pane),
                self.theme,
            ))
            .push(button(
                into_list,
                "List",
                Message::IntoList(pane),
                self.theme,
            ))
            .push(button(
                split_vertically,
                "|",
                Message::Split(pane_grid::Axis::Vertical, pane),
                self.theme,
            ));

        if total_panes > 1 {
            controls = controls.push(button(close, "x", Message::Close(pane), self.theme));
        }

        let content = Scrollable::new(scroll)
            .width(Length::Fill)
            .spacing(10)
            .align_items(Alignment::Start);

        let c = Column::new()
            .padding(2)
            .push(controls)
            .push(content)
            .align_items(Alignment::Start);

        Container::new(c)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .into()
    }
}
