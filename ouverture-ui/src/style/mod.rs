// use crate::fs;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

mod builtin;

pub mod stylesheet;

// pub async fn load_user_themes() -> Vec<Theme> {
//     log::debug!("loading user themes");
//
//     // fs::load_user_themes().await
// }

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Theme {
    pub base: BaseColors,
    pub normal: NormalColors,
    pub bright: BrightColors,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct BaseColors {
    #[serde(with = "serde_color")]
    pub background: iced_native::Color,
    #[serde(with = "serde_color")]
    pub foreground: iced_native::Color,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct NormalColors {
    #[serde(with = "serde_color")]
    pub primary: iced_native::Color,
    #[serde(with = "serde_color")]
    pub secondary: iced_native::Color,
    #[serde(with = "serde_color")]
    pub surface: iced_native::Color,
    #[serde(with = "serde_color")]
    pub error: iced_native::Color,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct BrightColors {
    #[serde(with = "serde_color")]
    pub primary: iced_native::Color,
    #[serde(with = "serde_color")]
    pub secondary: iced_native::Color,
    #[serde(with = "serde_color")]
    pub surface: iced_native::Color,
    #[serde(with = "serde_color")]
    pub error: iced_native::Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::dark()
    }
}

pub fn hex_to_color(hex: &str) -> Option<iced_native::Color> {
    if hex.len() == 7 {
        let hash = &hex[0..1];
        let r = u8::from_str_radix(&hex[1..3], 16);
        let g = u8::from_str_radix(&hex[3..5], 16);
        let b = u8::from_str_radix(&hex[5..7], 16);

        return match (hash, r, g, b) {
            ("#", Ok(r), Ok(g), Ok(b)) => Some(iced_native::Color {
                r: r as f32 / 255.0,
                g: g as f32 / 255.0,
                b: b as f32 / 255.0,
                a: 1.0,
            }),
            _ => None,
        };
    }

    None
}

pub fn color_to_hex(color: &iced_native::Color) -> String {
    let mut color_str = String::from("#");

    let iced_native::Color { r, g, b, .. } = color;
    color_str.push_str(&format!("{:02X}", (r * 255.0) as u8));
    color_str.push_str(&format!("{:02X}", (g * 255.0) as u8));
    color_str.push_str(&format!("{:02X}", (b * 255.0) as u8));

    color_str
}

//
// impl PartialEq for Theme {
//     fn eq(&self, other: &Self) -> bool {
//         self.name == other.name
//     }
// }
//
// impl PartialOrd for Theme {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.name.cmp(&other.name))
//     }
// }
//
// impl Eq for Theme {}
//
// impl Ord for Theme {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.name.cmp(&other.name)
//     }
// }
//
// Newtype on iced::Color so we can impl Deserialzer for it
struct Color(iced_native::Color);

mod serde_color {
    use super::Color;
    use super::{color_to_hex, hex_to_color};
    use serde::de::{self, Error, Unexpected, Visitor};
    use serde::ser;
    use std::fmt;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<iced_native::Color, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ColorVisitor;

        impl<'de> Visitor<'de> for ColorVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a hex string in the format of '#09ACDF'")
            }

            #[allow(clippy::unnecessary_unwrap)]
            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if let Some(color) = hex_to_color(s) {
                    return Ok(Color(color));
                }

                Err(de::Error::invalid_value(Unexpected::Str(s), &self))
            }
        }

        deserializer.deserialize_any(ColorVisitor).map(|c| c.0)
    }

    pub(crate) fn serialize<S>(color: &iced_native::Color, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&color_to_hex(color))
    }
}

#[cfg(test)]
mod tests {
    use super::{serde_color::deserialize, Theme};
    use serde::de::value::{Error, StrDeserializer};
    use serde::de::IntoDeserializer;

    #[test]
    fn test_hex_color_deser() {
        let colors = [
            "AABBCC", "AABBCG", "#AABBCG", "#AABB091", "#AABBCC", "#AABB09",
        ];

        for (idx, color_str) in colors.iter().enumerate() {
            let deserializer: StrDeserializer<Error> = color_str.into_deserializer();

            let color = deserialize(deserializer);

            if idx < 4 {
                assert!(color.is_err());
            } else {
                assert!(color.is_ok());
            }
        }
    }
    //
    // #[test]
    // fn test_hex_color_ser() {
    //     let color = super::NormalColors {
    //         primary: iced_native::Color::from_rgb(1.0, 1.0, 1.0),
    //         secondary: iced_native::Color::from_rgb(0.5, 0.6, 0.75789),
    //         surface: iced_native::Color::from_rgb(0.1, 0.2, 0.3),
    //         error: iced_native::Color::from_rgb(0.0, 0.0, 0.0),
    //     };
    //
    //     let ser = serde_yaml::to_string(&color).unwrap();
    //
    //     dbg!(&ser);
    // }
    //
    // #[test]
    // fn test_theme_yml_deser() {
    //     let theme_str = "---
    //     palette:
    //       base:
    //         background: '#484793'
    //         foreground: '#484793'
    //       normal:
    //         primary: '#484793'
    //         secondary: '#484793'
    //         surface: '#484793'
    //         error: '#484793'
    //       bright:
    //         primary: '#484793'
    //         secondary: '#484793'
    //         surface: '#484793'
    //         error: '#484793'
    //     ";
    //
    //     serde_yaml::from_str::<Theme>(theme_str).unwrap();
    // }
}
