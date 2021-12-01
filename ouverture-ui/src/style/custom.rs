use iced::{
    button, container, pane_grid, pane_grid::Line, radio, rule, scrollable, Background, Color,
    Vector,
};

use super::Themable;

#[derive(Debug, Clone, Copy)]
pub struct Custom {
    pub name : &'static str,

    pub background: Color,
    pub surface: Color,
    pub accent: Color,
    pub active: Color,
    pub scrollbar: Color,
    pub scroller: Color,
    pub hovered: Color,
}

impl PartialEq for Custom {
    fn eq(&self, other: &Custom) -> bool {
        self.name == other.name
    }
}

impl Eq for Custom {}

impl Default for Custom {
    fn default() -> Self {
        let background = Color::from_rgb(
            0x36 as f32 / 255.0,
            0x39 as f32 / 255.0,
            0x3F as f32 / 255.0,
        );

        let surface = Color::from_rgb(
            0x40 as f32 / 255.0,
            0x44 as f32 / 255.0,
            0x4B as f32 / 255.0,
        );

        let accent = Color::from_rgb(
            0x6F as f32 / 255.0,
            0xFF as f32 / 255.0,
            0xE9 as f32 / 255.0,
        );

        let active = Color::from_rgb(
            0x72 as f32 / 255.0,
            0x89 as f32 / 255.0,
            0xDA as f32 / 255.0,
        );

        let scrollbar = Color::from_rgb(
            0x2E as f32 / 255.0,
            0x33 as f32 / 255.0,
            0x38 as f32 / 255.0,
        );

        let scroller = Color::from_rgb(
            0x20 as f32 / 255.0,
            0x22 as f32 / 255.0,
            0x25 as f32 / 255.0,
        );

        let hovered = Color::from_rgb(
            0x67 as f32 / 255.0,
            0x7B as f32 / 255.0,
            0xC4 as f32 / 255.0,
        );

        let name = "Default";
        Custom {
            name,
            background,
            surface,
            accent,
            active,
            scrollbar,
            scroller,
            hovered
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Pane {
    theme: Custom,
    pub is_focused: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Scrollable {
    theme: Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rule{
    theme: Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Container{

    theme: Custom,

}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Radio{
    theme: Custom,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Primary(Custom),
    Destructive(Custom),
}

impl Button {
    fn default(theme: Custom) -> Self {
        Button::Primary(theme)
    }
}

impl Themable for Custom {
    fn name(&self) -> String {
        String::from("Custom")
    }

    fn into_container(self) -> Box<dyn container::StyleSheet> {
        Container::default().into()
    }
    fn into_radio(self) -> Box<dyn radio::StyleSheet> {
        Radio::default().into()
    }
    fn into_rule(self) -> Box<dyn rule::StyleSheet> {
        Rule::default().into()
    }
    fn into_scrollable(self) -> Box<dyn scrollable::StyleSheet> {
        Scrollable::default().into()
    }
    fn into_panegrid(self) -> Box<dyn pane_grid::StyleSheet> {
        Pane::default().into()
    }
    fn into_button(self) -> Box<dyn button::StyleSheet> {
        Button::default(self).into()
    }
}

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color {
                a: 0.99,
                ..self.theme.background
            }
            .into(),
            text_color: Color::WHITE.into(),
            ..container::Style::default()
        }
    }
}
impl pane_grid::StyleSheet for Pane {
    fn picked_split(&self) -> Option<Line> {
        None
    }
    fn hovered_split(&self) -> Option<Line> {
        None
    }
}
impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        let (background, text_color) = match self {
            Button::Primary(theme) => (Some(theme.active), Color::WHITE),
            Button::Destructive(theme) => (None, Color::from_rgb8(0xFF, 0x47, 0x47)),
        };

        button::Style {
            text_color,
            background: background.map(Background::Color),
            border_radius: 5.0,
            shadow_offset: Vector::new(0.0, 0.0),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        let background = match self {
            Button::Primary(theme) => Some(theme.hovered),
            Button::Destructive(_) => Some(Color {
                a: 0.2,
                ..active.text_color
            }),
        };

        button::Style {
            background: background.map(Background::Color),
            ..active
        }
    }
}

impl radio::StyleSheet for Radio {
    fn active(&self) -> radio::Style {
        radio::Style {
            background: self.theme.background.into(),
            dot_color: self.theme.active,
            border_width: 1.0,
            border_color: self.theme.active,
        }
    }

    fn hovered(&self) -> radio::Style {
        radio::Style {
            background: Color { a: 0.5, ..self.theme.surface }.into(),
            ..self.active()
        }
    }
}

impl scrollable::StyleSheet for Scrollable {
    fn active(&self) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Color {
                a: 0.8,
                ..self.theme.scrollbar
            }
            .into(),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: Color { a: 0.7, ..self.theme.scroller },
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self) -> scrollable::Scrollbar {
        let active = self.active();

        scrollable::Scrollbar {
            background: self.theme.scrollbar.into(),
            scroller: scrollable::Scroller {
                color: self.theme.scroller,
                ..active.scroller
            },
            ..active
        }
    }

    fn dragging(&self) -> scrollable::Scrollbar {
        let hovered = self.hovered();

        scrollable::Scrollbar {
            scroller: scrollable::Scroller {
                color: self.theme.accent,
                ..hovered.scroller
            },
            ..hovered
        }
    }
}

impl rule::StyleSheet for Rule {
    fn style(&self) -> rule::Style {
        rule::Style {
            color: self.theme.accent,
            width: 2,
            radius: 1.0,
            fill_mode: rule::FillMode::Percent(30.0),
        }
    }
}
