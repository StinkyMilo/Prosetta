mod wind_renderer;

#[derive(PartialEq, Clone, Copy)]
pub enum TermColor {
    Black,
    Blue,
    Cyan,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

pub trait Renderer {
    fn set_color(&mut self, color: (TermColor, bool));
    fn add(&mut self, text: &str);
    fn push(&mut self, text: char);
    fn add_with(&mut self, text: &str, color: (TermColor, bool));
    fn push_with(&mut self, text: char, color: (TermColor, bool));
    fn into_string(self: Box<Self>) -> String;
}
