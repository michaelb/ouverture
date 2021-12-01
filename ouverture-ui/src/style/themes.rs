use super::custom::Custom;


impl Custom {
    pub fn new(name: &str) -> Self {
        match name {
            "solarized" => {solarized()},
            _ => Custom::default()
        }
    }

}



fn solarized() -> Custom {
    Custom {
        name: "solarized",
        background: [0.9,0.9,0.21].into(),
        accent: [0.9,0.9,0.21].into(),
        hovered: [0.9,0.9,0.21].into(),
        surface:  [0.9,0.9,0.21].into(),
        active: [0.9,0.9,0.21].into(),
        scrollbar: [0.9,0.9,0.21].into(),
        scroller: [0.9,0.9,0.21].into(),
    }
}
