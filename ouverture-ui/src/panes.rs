use iced::executor;
use iced::keyboard;
use iced::theme::Theme;
use iced::widget::pane_grid::{self, PaneGrid};
use iced::widget::{button, column, container, scrollable, text};
use iced::{Application, Command, Element, Length};

pub struct Panes {
    panes: pane_grid::State<Box<dyn Content>>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
    config: Config,
}

use crate::config::Config;
use crate::Message;
use log::{debug, trace, warn};

use std::any::Any;

mod control_bar;
// pub mod list;
pub mod list;
mod menu;

impl Panes {
    pub fn new(conf: &Config) -> Self {
        let a: Box<dyn Content> = Box::new(Editor::new(0));
        let b: Box<dyn Content> = Box::new(control_bar::ControlBar::new(&conf));

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
            config: conf.clone(),
        }
    }
}

impl Application for Panes {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = Config;
    type Theme = Theme;

    fn new(c: Config) -> (Self, Command<Message>) {
        (Panes::new(&c), Command::none())
    }

    fn title(&self) -> String {
        String::from("Pane grid - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        use Message::*;
        match message {
            Split(axis, pane) => {
                let result =
                    self.panes
                        .split(axis, &pane, Box::new(Editor::new(self.panes_created)));

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }

                self.panes_created += 1;
            }
            SplitFocused(axis) => {
                if let Some(pane) = self.focus {
                    let result =
                        self.panes
                            .split(axis, &pane, Box::new(Editor::new(self.panes_created)));

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
                if let iced::widget::pane_grid::Target::Pane(dest_pane, _region) = &target {
                    self.panes.swap(&pane, &dest_pane);
                } else {
                    debug!("dragged a pane on an edge: doing nothing");
                }
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
                let menu = menu::Menu::new();
                let result = self
                    .panes
                    .split(pane_grid::Axis::Horizontal, &pane, Box::new(menu));

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }
                self.panes.close(&pane);
            }
            IntoList(pane) => {
                let list = list::List::new(self.config.clone());
                let result = self
                    .panes
                    .split(pane_grid::Axis::Horizontal, &pane, Box::new(list));

                if let Some((new_pane, _)) = result {
                    self.focus = Some(new_pane);
                    self.panes.close(&pane);
                    return Command::single(Message::AskRefreshList(new_pane).into());
                } else {
                    warn!("failed to close pane, keeping current one");
                };
            }

            IntoControlBar(pane) => {
                let menu = control_bar::ControlBar::new(&self.config);
                let result = self
                    .panes
                    .split(pane_grid::Axis::Horizontal, &pane, Box::new(menu));

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }
                self.panes.close(&pane);
            }
            AskRefreshList(pane) => {
                let list: &mut list::List = self
                    .panes
                    .get_mut(&pane)
                    .unwrap()
                    .as_any_mut()
                    .downcast_mut::<list::List>()
                    .unwrap();
                return list.update(AskRefreshList(pane));
            }
            ReceivedNewList(pane, reply) => {
                let list: &mut list::List = self
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
            msg => {
                let command =
                    Command::batch(self.panes.iter_mut().map(|(_p, s)| s.update(msg.clone())));
                debug!("passing message to children");
                return command;
            }
        }

        Command::none()
    }
    fn view(&self) -> Element<Message> {
        let focus = self.focus;
        let total_panes = self.panes.len();

        PaneGrid::new(&self.panes, |pane, content, _is_maximized| {
            let is_focused = focus == Some(pane);

            let title_bar: pane_grid::TitleBar<Message> = if is_focused {
                pane_grid::TitleBar::new(text("focused")).padding(10)
            } else {
                pane_grid::TitleBar::new(text("not focused")).padding(10)
            };
            trace!("updating view in panes");
            pane_grid::Content::new(content.view(pane, total_panes)).title_bar(title_bar)
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
    fn view(&self, pane: pane_grid::Pane, total_panes: usize) -> Element<Message>;
    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

struct Editor {}

impl Editor {
    fn new(_id: usize) -> Self {
        Editor {}
    }
}
impl Content for Editor {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn view(&self, pane: pane_grid::Pane, total_panes: usize) -> Element<Message> {
        let mut controls = column![].spacing(5).max_width(150);
        controls = controls
            .push(button(text("-")).on_press(Message::Split(pane_grid::Axis::Horizontal, pane)))
            .push(button(text("Menu")).on_press(Message::IntoMenu(pane)))
            .push(button(text("ControlBar")).on_press(Message::IntoControlBar(pane)))
            .push(button(text("List")).on_press(Message::IntoList(pane)))
            .push(button(text("|")).on_press(Message::Split(pane_grid::Axis::Vertical, pane)));

        if total_panes > 1 {
            controls = controls.push(button(text("x")).on_press(Message::Close(pane)));
        }

        container(scrollable(controls))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .into()
    }
}
