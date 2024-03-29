use iced::theme::{self, Theme};
use iced::Color;

pub fn hex_to_color(hex: &str) -> Option<Color> {
    if hex.len() == 7 {
        let hash = &hex[0..1];
        let r = u8::from_str_radix(&hex[1..3], 16);
        let g = u8::from_str_radix(&hex[3..5], 16);
        let b = u8::from_str_radix(&hex[5..7], 16);

        return match (hash, r, g, b) {
            ("#", Ok(r), Ok(g), Ok(b)) => Some(Color {
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

pub fn color_to_hex(color: &iced::Color) -> String {
    let mut color_str = String::from("#");

    let iced::Color { r, g, b, .. } = color;
    color_str.push_str(&format!("{:02X}", (r * 255.0) as u8));
    color_str.push_str(&format!("{:02X}", (g * 255.0) as u8));
    color_str.push_str(&format!("{:02X}", (b * 255.0) as u8));

    color_str
}

pub struct BuiltinTheme;

impl BuiltinTheme {
    pub fn all() -> Vec<(String, Theme)> {
        vec![
            ("alliance".to_string(), BuiltinTheme::alliance()),
            ("dark".to_string(), BuiltinTheme::dark()),
            ("light".to_string(), BuiltinTheme::light()),
            // ("ayu".to_string(), BuiltinTheme::ayu()),
            // ("dracula".to_string(), BuiltinTheme::dracula()),
            // ("ferra".to_string(), BuiltinTheme::ferra()),
            // ("forest Night".to_string(), BuiltinTheme::forest_night()),
            // ("gruvbox".to_string(), BuiltinTheme::gruvbox()),
            // ("horde".to_string(), BuiltinTheme::horde()),
            // ("nord".to_string(), BuiltinTheme::nord()),
            // ("one Dark".to_string(), BuiltinTheme::one_dark()),
            // ("outrun".to_string(), BuiltinTheme::outrun()),
            // ("solarized Dark".to_string(), BuiltinTheme::solarized_dark()),
            // ("solarized Light".to_string(), BuiltinTheme::solarized_light()),
            // ("sort".to_string(), BuiltinTheme::sort()),
        ]
    }

    pub fn dark() -> Theme {
        Theme::Dark
    }

    pub fn light() -> Theme {
        Theme::Light
    }

    pub fn alliance() -> Theme {
        Theme::custom(theme::Palette {
            background: hex_to_color("#111111").unwrap(),
            primary: hex_to_color("#3f2b56").unwrap(),
            success: hex_to_color("#4a3c1c").unwrap(),
            text: hex_to_color("#828282").unwrap(),
            danger: hex_to_color("#992B2B").unwrap(),
        })
    }
    //
    //     pub fn horde() -> Theme {
    //         Theme {
    //             base: BaseColors {
    //                 background: hex_to_color("#161313").unwrap(),
    //                 foreground: hex_to_color("#211C1C").unwrap(),
    //             },
    //             normal: NormalColors {
    //                 primary: hex_to_color("#331E1F").unwrap(),
    //                 secondary: hex_to_color("#542A18").unwrap(),
    //                 surface: hex_to_color("#5E5B5A").unwrap(),
    //                 error: hex_to_color("#44282a").unwrap(),
    //             },
    //             bright: BrightColors {
    //                 primary: hex_to_color("#953e43").unwrap(),
    //                 secondary: hex_to_color("#e27342").unwrap(),
    //                 surface: hex_to_color("#9B9897").unwrap(),
    //                 error: hex_to_color("#953e43").unwrap(),
    //             },
    //         }
    //     }
    //
    //     pub fn ayu() -> Theme {
    //         Theme {
    //             base: BaseColors {
    //                 background: hex_to_color("#1f2430").unwrap(),
    //                 foreground: hex_to_color("#232834").unwrap(),
    //             },
    //             normal: NormalColors {
    //                 primary: hex_to_color("#987a47").unwrap(),
    //                 secondary: hex_to_color("#315e6b").unwrap(),
    //                 surface: hex_to_color("#60697a").unwrap(),
    //                 error: hex_to_color("#712a34").unwrap(),
    //             },
    //             bright: BrightColors {
    //                 primary: hex_to_color("#ffcc66").unwrap(),
    //                 secondary: hex_to_color("#5ccfe6").unwrap(),
    //                 surface: hex_to_color("#cbccc6").unwrap(),
    //                 error: hex_to_color("#ff3333").unwrap(),
    //             },
    //         }
    //     }
    //
    //     pub fn dracula() -> Theme {
    //         Theme {
    //             base: BaseColors {
    //                 background: hex_to_color("#282a36").unwrap(),
    //                 foreground: hex_to_color("#353746").unwrap(),
    //             },
    //             normal: NormalColors {
    //                 primary: hex_to_color("#483e61").unwrap(),
    //                 secondary: hex_to_color("#386e50").unwrap(),
    //                 surface: hex_to_color("#a2a4a3").unwrap(),
    //                 error: hex_to_color("#A13034").unwrap(),
    //             },
    //             bright: BrightColors {
    //                 primary: hex_to_color("#bd94f9").unwrap(),
    //                 secondary: hex_to_color("#49eb7a").unwrap(),
    //                 surface: hex_to_color("#f4f8f3").unwrap(),
    //                 error: hex_to_color("#ff7ac6").unwrap(),
    //             },
    //         }
    //     }
    //
    //     pub fn forest_night() -> Theme {
    //         Theme {
    //             base: BaseColors {
    //                 background: hex_to_color("#323d43").unwrap(),
    //                 foreground: hex_to_color("#3c474d").unwrap(),
    //             },
    //             normal: NormalColors {
    //                 primary: hex_to_color("#505a60").unwrap(),
    //                 secondary: hex_to_color("#465258").unwrap(),
    //                 surface: hex_to_color("#999f93").unwrap(),
    //                 error: hex_to_color("#74484c").unwrap(),
    //             },
    //             bright: BrightColors {
    //                 primary: hex_to_color("#a7c080").unwrap(),
    //                 secondary: hex_to_color("#83b6af").unwrap(),
    //                 surface: hex_to_color("#d8caac").unwrap(),
    //                 error: hex_to_color("#e68183").unwrap(),
    //             },
    //         }
    //     }
    //
    //     pub fn gruvbox() -> Theme {
    //         Theme {
    //             base: BaseColors {
    //                 background: hex_to_color("#282828").unwrap(),
    //                 foreground: hex_to_color("#3c3836").unwrap(),
    //             },
    //             normal: NormalColors {
    //                 primary: hex_to_color("#63612f").unwrap(),
    //                 secondary: hex_to_color("#695133").unwrap(),
    //                 surface: hex_to_color("#928374").unwrap(),
    //                 error: hex_to_color("#81302e").unwrap(),
    //             },
    //             bright: BrightColors {
    //                 primary: hex_to_color("#98971a").unwrap(),
    //                 secondary: hex_to_color("#d79921").unwrap(),
    //                 surface: hex_to_color("#ebdbb2").unwrap(),
    //                 error: hex_to_color("#cc241d").unwrap(),
    //             },
    //         }
    //     }
    //
    //     pub fn nord() -> Theme {
    //         Theme {
    //             base: BaseColors {
    //                 background: hex_to_color("#2e3440").unwrap(),
    //                 foreground: hex_to_color("#3b4252").unwrap(),
    //             },
    //             normal: NormalColors {
    //                 primary: hex_to_color("#485b60").unwrap(),
    //                 secondary: hex_to_color("#425066").unwrap(),
    //                 surface: hex_to_color("#9196a1").unwrap(),
    //                 error: hex_to_color("#894f5a").unwrap(),
    //             },
    //             bright: BrightColors {
    //                 primary: hex_to_color("#8fbcbb").unwrap(),
    //                 secondary: hex_to_color("#5e81ac").unwrap(),
    //                 surface: hex_to_color("#eceff4").unwrap(),
    //                 error: hex_to_color("#bf616a").unwrap(),
    //             },
    //         }
    //     }
    //
    //     pub fn outrun() -> Theme {
    //         Theme {
    //             base: BaseColors {
    //                 background: hex_to_color("#0d0821").unwrap(),
    //                 foreground: hex_to_color("#110A2B").unwrap(),
    //             },
    //             normal: NormalColors {
    //                 primary: hex_to_color("#330442").unwrap(),
    //                 secondary: hex_to_color("#6e3e2e").unwrap(),
    //                 surface: hex_to_color("#484e81").unwrap(),
    //                 error: hex_to_color("#671a30").unwrap(),
    //             },
    //             bright: BrightColors {
    //                 primary: hex_to_color("#ff00ff").unwrap(),
    //                 secondary: hex_to_color("#ff963a").unwrap(),
    //                 surface: hex_to_color("#757dc8").unwrap(),
    //                 error: hex_to_color("#db2c3e").unwrap(),
    //             },
    //         }
    //     }
    //
    //     pub fn solarized_dark() -> Theme {
    //         Theme {
    //             base: BaseColors {
    //                 background: hex_to_color("#012b36").unwrap(),
    //                 foreground: hex_to_color("#093642").unwrap(),
    //             },
    //             normal: NormalColors {
    //                 primary: hex_to_color("#1A615B").unwrap(),
    //                 secondary: hex_to_color("#523F09").unwrap(),
    //                 surface: hex_to_color("#63797e").unwrap(),
    //                 error: hex_to_color("#b80f15").unwrap(),
    //             },
    //             bright: BrightColors {
    //                 primary: hex_to_color("#2aa096").unwrap(),
    //                 secondary: hex_to_color("#a37f12").unwrap(),
    //                 surface: hex_to_color("#93a1a1").unwrap(),
    //                 error: hex_to_color("#EE2F36").unwrap(),
    //             },
    //         }
    //     }
    //
    //     pub fn solarized_light() -> Theme {
    //         Theme {
    //             base: BaseColors {
    //                 background: hex_to_color("#fdf6e3").unwrap(),
    //                 foreground: hex_to_color("#eee8d5").unwrap(),
    //             },
    //             normal: NormalColors {
    //                 primary: hex_to_color("#BCCCC3").unwrap(),
    //                 secondary: hex_to_color("#ccbd9e").unwrap(),
    //                 surface: hex_to_color("#95a3a2").unwrap(),
    //                 error: hex_to_color("#b80f15").unwrap(),
    //             },
    //             bright: BrightColors {
    //                 primary: hex_to_color("#2aa096").unwrap(),
    //                 secondary: hex_to_color("#a37f12").unwrap(),
    //                 surface: hex_to_color("#4C5D63").unwrap(),
    //                 error: hex_to_color("#EE2F36").unwrap(),
    //             },
    //         }
    //     }
    //
    //     pub fn sort() -> Theme {
    //         Theme {
    //             base: BaseColors {
    //                 background: hex_to_color("#000000").unwrap(),
    //                 foreground: hex_to_color("#101010").unwrap(),
    //             },
    //             normal: NormalColors {
    //                 primary: hex_to_color("#525252").unwrap(),
    //                 secondary: hex_to_color("#525252").unwrap(),
    //                 surface: hex_to_color("#525252").unwrap(),
    //                 error: hex_to_color("#525252").unwrap(),
    //             },
    //             bright: BrightColors {
    //                 primary: hex_to_color("#A3A3A3").unwrap(),
    //                 secondary: hex_to_color("#A3A3A3").unwrap(),
    //                 surface: hex_to_color("#A3A3A3").unwrap(),
    //                 error: hex_to_color("#A3A3A3").unwrap(),
    //             },
    //         }
    //     }
    //
    //     pub fn ferra() -> Theme {
    //         Theme {
    //             base: BaseColors {
    //                 background: hex_to_color("#211f22").unwrap(),
    //                 foreground: hex_to_color("#2b292d").unwrap(),
    //             },
    //             normal: NormalColors {
    //                 primary: hex_to_color("#664A50").unwrap(),
    //                 secondary: hex_to_color("#855859").unwrap(),
    //                 surface: hex_to_color("#816961").unwrap(),
    //                 error: hex_to_color("#713f47").unwrap(),
    //             },
    //             bright: BrightColors {
    //                 primary: hex_to_color("#b4838d").unwrap(),
    //                 secondary: hex_to_color("#e5989b").unwrap(),
    //                 surface: hex_to_color("#fecdb2").unwrap(),
    //                 error: hex_to_color("#e06b75").unwrap(),
    //             },
    //         }
    //     }
    //
    //     pub fn one_dark() -> Theme {
    //         Theme {
    //             base: BaseColors {
    //                 background: hex_to_color("#282c34").unwrap(),
    //                 foreground: hex_to_color("#2c323c").unwrap(),
    //             },
    //             normal: NormalColors {
    //                 primary: hex_to_color("#385c7c").unwrap(),
    //                 secondary: hex_to_color("#654473").unwrap(),
    //                 surface: hex_to_color("#5b626e").unwrap(),
    //                 error: hex_to_color("#713f47").unwrap(),
    //             },
    //             bright: BrightColors {
    //                 primary: hex_to_color("#61afef").unwrap(),
    //                 secondary: hex_to_color("#c679dd").unwrap(),
    //                 surface: hex_to_color("#a6adba").unwrap(),
    //                 error: hex_to_color("#e06b75").unwrap(),
    //             },
    //         }
    //     }
}
