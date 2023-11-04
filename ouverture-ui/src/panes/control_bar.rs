use iced::{Command, Element, Length};

use iced::widget::{button, column, container, pane_grid, row, slider, text};

use super::Content;
use crate::config::Config;
use crate::Message;
use ouverture_core::music::song::Song;

use ouverture_core::server::Reply;

use log::debug;
pub struct ControlBar {
    slider_value: u32,
    current_song_length: Option<u64>, // length in milliseconds
    config: Config,
}
use iced_runtime::command::Action;

use ouverture_core::server::Command as ServerCommand;
use ouverture_core::server::Server;

impl ControlBar {
    pub fn new(c: &Config) -> Self {
        ControlBar {
            slider_value: 0, // between 0 and 4096
            current_song_length: None,
            config: c.clone(),
        }
    }

    pub fn notify_seek(&mut self, value: u32) -> Command<Message> {
        let address =
            self.config.server_address.to_string() + ":" + &self.config.server_port.to_string();
        debug!("address is {address}");
        self.slider_value = value;

        Command::single(Action::Future(Box::pin(async move {
            Server::send_wait(&ServerCommand::Seek((value as f32) / 4096f32), &address)
                .await
                .unwrap();
            debug!("asked for list refresh");
            Message::Nothing
        })))
    }

    pub fn refresh(&self) -> Command<Message> {
        let address =
            self.config.server_address.to_string() + ":" + &self.config.server_port.to_string();
        debug!("refreshing control");

        Command::single(Action::Future(Box::pin(async move {
            let reply = Server::send_wait(&ServerCommand::GetCurrentSong, &address)
                .await
                .unwrap();
            debug!("asked for new current song, got {reply:?}");
            match reply {
                Reply::CurrentSong(song, seek) => Message::ReceivedNewCurrentSong(song, seek),
                _ => Message::Nothing,
            }
        })))
    }

    pub fn refresh_from_song(
        &mut self,
        opt_song: Option<Song>,
        opt_seek: Option<f32>,
    ) -> Command<Message> {
        if let Some(song) = opt_song {
            debug!("recevied song length : {:?}", song.duration);
            self.current_song_length = Some(song.duration.as_millis() as u64);
        }
        if let Some(seek) = opt_seek {
            self.slider_value = (seek * 4096f32) as u32;
        }
        Command::single(Message::SliderChangedAuto(self.slider_value).into())
    }
}

impl Content for ControlBar {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::RefreshControl(_) => self.refresh(),
            Message::SliderChanged(value) => self.notify_seek(value),
            Message::ReceivedNewCurrentSong(song, seek) => self.refresh_from_song(song, Some(seek)),
            _ => Command::none(),
        }
    }
    fn view(&self, _pane: pane_grid::Pane, _total_panes: usize) -> Element<Message> {
        let slider = container(slider(0..=4096, self.slider_value, |x| {
            Message::SliderChanged(x)
        }))
        .width(250);

        let button_controls = row![]
            .spacing(5)
            .push(button(text("<-")).on_press(Message::Previous))
            .push(button(text(">")).on_press(Message::Toggle))
            .push(button(text("->")).on_press(Message::Next));
        let controls = column![].spacing(15).push(button_controls).push(slider);

        container(controls)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .into()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
