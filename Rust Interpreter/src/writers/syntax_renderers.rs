pub(crate) mod wind_renderer;

#[allow(dead_code)]
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

pub trait Renderer: Default {
    fn set_color(&mut self, color: (TermColor, bool));
    fn add(&mut self, text: &[u8]);
    fn push(&mut self, text: u8);
    fn add_with(&mut self, text: &[u8], color: (TermColor, bool));
    fn push_with(&mut self, text: u8, color: (TermColor, bool));
    fn into_string(self) -> Vec<u8>;
}
