use iced::{Element, Length, Command};

use iced::widget::{button, container, pane_grid, row, text};

use super::Content;
use crate::Message;

use log::debug;
pub struct ControlBar {
    currently_playing: bool,
}
use iced_native::command::Action;

use ouverture_core::music::song::Song;
use ouverture_core::server::Command as ServerCommand;
use ouverture_core::server::Server;

impl Content for ControlBar {
    fn view(&self, _pane: pane_grid::Pane, _total_panes: usize) -> Element<Message> {
        self.view()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl ControlBar {
    pub fn new() -> Self {
        ControlBar {
            currently_playing: false,
        }
    }
    fn update(&mut self, message:Message) -> Command<Message> {
        let address = "127.0.0.1:6603";

        match message {
            Message::Play(opt_song) => Command::single(Action::Future(Box::pin(async move {
                let reply = Server::send_wait(&ServerCommand::Toggle, address)
                    .await
                    .unwrap();
                debug!("GUI asked for play");
                Message::Nothing
            }))),
            Message::Toggle => Command::single(Action::Future(Box::pin(async move {
                debug!("GUI asking for toggle");
                let reply = Server::send_wait(&ServerCommand::Toggle, address)
                    .await
                    .unwrap();
                debug!("GUI asked for toggle, server replied: {:?}", reply);
                Message::Nothing
            }))),
            _ => Command::none()
        }
    }

    fn view(&self) -> Element<Message> {
        let ControlBar {
            currently_playing: _,
        } = self;

        let controls = row![]
            .spacing(5)
            .push(button(text("<-")).on_press(Message::Previous))
            .push(button(text(">")).on_press(Message::Toggle))
            .push(button(text("->")).on_press(Message::Next));

        container(controls)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .into()
    }
}
