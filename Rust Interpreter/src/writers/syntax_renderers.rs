pub(crate) mod html_renderer;
pub(crate) mod wind_renderer;
pub(crate) mod line_renderer;

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

#[allow(dead_code)]
pub trait Renderer: Default {
    type Output;
    // fn set_color(&mut self, color: (TermColor, bool));
    fn add(&mut self, text: &[u8]);
    fn push(&mut self, text: u8);
    fn add_with(&mut self, text: &[u8], color: (TermColor, bool));
    fn add_with_mult(&mut self, text: &[u8], color: Vec<(TermColor, bool)>);
    fn push_with(&mut self, text: u8, color: (TermColor, bool));
    fn into_data(self) -> Self::Output;
}
