use iced::{container, pane_grid, radio,button, rule, scrollable, Background, Color};
use super::Themable;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Light {}

impl Themable for Light {
    fn name(&self) -> String {
        String::from("Light")
    }
    fn into_container(self) -> Box<dyn container::StyleSheet> {
        Default::default()
    }
    fn into_radio(self) -> Box<dyn radio::StyleSheet> {
        Default::default()
    }
    fn into_scrollable(self) -> Box<dyn scrollable::StyleSheet> {
        Default::default()
    }
    fn into_rule(self) -> Box<dyn rule::StyleSheet> {
        Default::default()
    }
    fn into_panegrid(self) -> Box<dyn pane_grid::StyleSheet> {
        Default::default()
    }
    fn into_button(self) -> Box<dyn button::StyleSheet> {
        Default::default()
    }
}
