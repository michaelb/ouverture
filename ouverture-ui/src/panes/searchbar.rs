use iced::{
    button, executor, pane_grid, scrollable, slider, Application, Button, Clipboard, Column,
    Command, Container, Element, Length, ProgressBar, Radio, Row, Rule, Sandbox, Scrollable,
    Settings, Slider, Space, Text, Align
};



use super::style;


#[derive(Default)]
struct Progress {
    value: f32,
    progress_bar_slider: slider::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SliderChanged(f32),
}

impl Sandbox for Progress {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("A simple Progressbar")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SliderChanged(x) => self.value = x,
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .push(ProgressBar::new(0.0..=100.0, self.value))
            .push(
                Slider::new(
                    &mut self.progress_bar_slider,
                    0.0..=100.0,
                    self.value,
                    Message::SliderChanged,
                )
                .step(0.01),
            )
            .into()
    }
}
