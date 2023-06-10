mod builtin;

use builtin::BuiltinTheme;
use iced::theme::Theme;

use fixedstr::str32;
use log::warn;

#[derive(Debug, Clone, Default)]
pub struct ThemeType(str32);


impl Into<Theme> for ThemeType {
    fn into(self) -> Theme {
        for (name, theme) in BuiltinTheme::all() {
            if name == self.0.to_str() {
                return theme;
            }
        }
        warn!("Theme {self:?} was not found among the builtin themes");
        Theme::default()
    }
}


impl Into<ThemeType> for String {
    fn into(self) -> ThemeType {
        return ThemeType(str32::from(self));
    }
}
