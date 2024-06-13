use super::{Renderer, TermColor};

struct WindowsRenderer {
    str: String,
    old_color: (TermColor, bool),
    new_color: (TermColor, bool),
}
impl Renderer for WindowsRenderer {
    fn set_color(&mut self, color: (TermColor, bool)) {
        self.new_color = color;
    }

    fn add(&mut self, text: &str) {
        self.check_color();
        self.str.push_str(text);
    }

    fn push(&mut self, char: char) {
        self.check_color();
        self.str.push(char);
    }

    fn add_with(&mut self, text: &str, color: (TermColor, bool)) {
        self.set_color(color);
        self.add(text);
    }

    fn push_with(&mut self, char: char, color: (TermColor, bool)) {
        self.set_color(color);
        self.push(char);
    }

    fn into_string(self: Box<Self>) -> String {
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
    
    fn change_color(str: &mut String, color: (TermColor, bool)) {
        *str += "\x1b[";
        *str += match color {
            (TermColor::Black, false) => "30",
            (TermColor::Red, false) => "31",
            (TermColor::Green, false) => "32",
            (TermColor::Yellow, false) => "33",
            (TermColor::Blue, false) => "34",
            (TermColor::Purple, false) => "35",
            (TermColor::Cyan, false) => "36",
            (TermColor::White, false) => "37",
            (TermColor::Black, true) => "90",
            (TermColor::Red, true) => "91",
            (TermColor::Green, true) => "92",
            (TermColor::Yellow, true) => "93",
            (TermColor::Blue, true) => "94",
            (TermColor::Purple, true) => "95",
            (TermColor::Cyan, true) => "96",
            (TermColor::White, true) => "97",
        };
        *str += "m";
    }
}
