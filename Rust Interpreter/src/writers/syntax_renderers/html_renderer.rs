use super::{Renderer, TermColor};

pub struct HTMLRenderer {
    str: Vec<u8>,
    old_color: (TermColor, bool),
    new_color: (TermColor, bool),
}

impl Default for HTMLRenderer {
    fn default() -> Self {
        let color = (TermColor::White, true);
        Self {
            str: Vec::new(),
            old_color: color,
            new_color: color,
        }
    }
}

impl Renderer for HTMLRenderer {
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

const BASE_COLOR: (TermColor, bool) = (TermColor::White, true);

impl HTMLRenderer {
    fn check_color(&mut self) {
        if self.old_color != self.new_color {
            Self::change_color(&mut self.str, self.new_color, self.old_color);
            self.old_color = self.new_color;
        }
    }

    fn change_color(str: &mut Vec<u8>, new_color: (TermColor, bool), old_color: (TermColor, bool)) {
        if old_color != BASE_COLOR {
            str.extend_from_slice(b"</span>");
        }
        if new_color != BASE_COLOR {
            str.extend_from_slice(b"<span class=\"term_");
            if new_color.1 {
                str.extend_from_slice(b"b_");
            }
            str.extend_from_slice(match new_color.0 {
                TermColor::Black => b"black",
                TermColor::Red => b"red",
                TermColor::Green => b"green",
                TermColor::Yellow => b"yellow",
                TermColor::Blue => b"blue",
                TermColor::Purple => b"purple",
                TermColor::Cyan => b"cyan",
                TermColor::White => b"white",
            });
            str.extend_from_slice(b"\">")
        }
    }
}
