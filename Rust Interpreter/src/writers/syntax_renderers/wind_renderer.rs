use super::{Renderer, TermColor};

pub struct WindowsRenderer {
    str: Vec<u8>,
    old_color: (TermColor, bool),
    new_color: (TermColor, bool),
}

impl Default for WindowsRenderer {
    fn default() -> Self {
        let color = (TermColor::White, true);
        Self {
            str: Vec::new(),
            old_color: color,
            new_color: color,
        }
    }
}

impl Renderer for WindowsRenderer {
    fn set_color(&mut self, color: (TermColor, bool)) {
        self.new_color = color;
    }

    fn add(&mut self, text: &[u8]) {
        self.check_color();
        self.str.extend_from_slice(text);
    }

    fn push(&mut self, char: u8) {
        self.check_color();
        self.str.push(char);
    }

    fn add_with(&mut self, text: &[u8], color: (TermColor, bool)) {
        self.set_color(color);
        self.add(text);
    }

    fn add_with_mult(&mut self, text: &[u8], colors: Vec<(TermColor, bool)>) {
        for color in &colors[..colors.len() - 1] {
            self.set_color(*color);
            self.push(b'_');
        }
        self.set_color(*colors.last().unwrap());
        self.add(text);
    }

    fn push_with(&mut self, char: u8, color: (TermColor, bool)) {
        self.set_color(color);
        self.push(char);
    }

    fn into_string(self) -> Vec<u8> {
        self.str
    }
}
impl WindowsRenderer {
    fn check_color(&mut self) {
        if self.old_color != self.new_color {
            Self::change_color(&mut self.str, self.new_color);
            self.old_color = self.new_color;
        }
    }

    fn change_color(str: &mut Vec<u8>, color: (TermColor, bool)) {
        str.extend_from_slice(b"\x1b[");
        str.extend_from_slice(match color {
            (TermColor::Black, false) => b"30",
            (TermColor::Red, false) => b"31",
            (TermColor::Green, false) => b"32",
            (TermColor::Yellow, false) => b"33",
            (TermColor::Blue, false) => b"34",
            (TermColor::Purple, false) => b"35",
            (TermColor::Cyan, false) => b"36",
            (TermColor::White, false) => b"37",
            (TermColor::Black, true) => b"90",
            (TermColor::Red, true) => b"91",
            (TermColor::Green, true) => b"92",
            (TermColor::Yellow, true) => b"93",
            (TermColor::Blue, true) => b"94",
            (TermColor::Purple, true) => b"95",
            (TermColor::Cyan, true) => b"96",
            (TermColor::White, true) => b"97",
        });
        str.push(b'm');
    }
}
