use enum_dispatch::enum_dispatch;
use iced::{button, container, pane_grid, radio, rule, scrollable, Color};
use std::cmp::Eq;
mod custom;
mod light;
mod themes;

#[enum_dispatch]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light(light::Light),
    Custom(custom::Custom),
}

#[enum_dispatch(Theme)]
trait Themable {
    fn name(&self) -> String;

    fn into_container(self) -> Box<dyn container::StyleSheet>;
    fn into_radio(self) -> Box<dyn radio::StyleSheet>;
    fn into_rule(self) -> Box<dyn rule::StyleSheet>;
    fn into_scrollable(self) -> Box<dyn scrollable::StyleSheet>;
    fn into_panegrid(self) -> Box<dyn pane_grid::StyleSheet>;
    fn into_button(self) -> Box<dyn button::StyleSheet>;
}

#[derive(Debug, Clone, Copy)]
pub struct CustomTheme {
    name: &'static str,
    background: Color,
    surface: Color,
    accent: Color,
    active: Color,
    scrollbar: Color,
    scroller: Color,
}

impl PartialEq for CustomTheme {
    fn eq(&self, other: &CustomTheme) -> bool {
        self.name == other.name
    }
}

impl Eq for CustomTheme {}

impl Theme {
    pub fn build_all() -> Vec<Theme> {
        vec![Theme::Custom(custom::Custom::default())]
        // add custom themes TODO
    }
}
impl Default for Theme {
    fn default() -> Theme {
        Theme::Custom(custom::Custom::new("solarized"))
        // Theme::Custom(custom::Custom::default())
        // Theme::Light(light::Light{})
    }
}
//
impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(theme: Theme) -> Self {
        theme.into_container()
    }
}

impl From<Theme> for Box<dyn radio::StyleSheet> {
    fn from(theme: Theme) -> Self {
        theme.into_radio()
    }
}

impl From<Theme> for Box<dyn scrollable::StyleSheet> {
    fn from(theme: Theme) -> Self {
        theme.into_scrollable()
    }
}

impl From<Theme> for Box<dyn rule::StyleSheet> {
    fn from(theme: Theme) -> Self {
        theme.into_rule()
    }
}

impl From<Theme> for Box<dyn pane_grid::StyleSheet> {
    fn from(theme: Theme) -> Self {
        theme.into_panegrid()
    }
}

impl From<Theme> for Box<dyn button::StyleSheet> {
    fn from(theme: Theme) -> Self {
        theme.into_button()
    }
}
