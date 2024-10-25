use super::{Renderer, TermColor};
use bstr::ByteSlice;

pub struct LineRenderer {
    vec: Vec<Highlight>,
    curr_line: usize,
    curr_index: usize,
    old_color: Vec<(TermColor, bool)>,
    new_color: Vec<(TermColor, bool)>,
}

impl Default for LineRenderer {
    fn default() -> Self {
        let color = vec![(TermColor::White, true)];
        Self {
            vec: Vec::new(),
            old_color: color.clone(),
            new_color: color,
            curr_line: 0,
            curr_index: 0,
        }
    }
}

#[derive(Debug)]
pub struct Highlight {
    line: usize,
    index: usize,
    length: usize,
    color: Vec<&'static str>,
}

impl Renderer for LineRenderer {
    type Output = Vec<Highlight>;

    fn add(&mut self, text: &[u8]) {
        let splited = text.split_str(b"\n");
        let mut first = true;
        for line in splited {
            if !first {
                self.curr_line += 1;
                self.curr_index = 0;
            }
            self.change_or_add(line);
            first = false;
        }
    }

    fn push(&mut self, char: u8) {
        if char == b'\n' {
            self.curr_line += 1;
            self.curr_index = 0;
        } else {
            self.change_or_add(&[char]);
        }
    }

    fn add_with(&mut self, text: &[u8], color: (TermColor, bool)) {
        self.new_color = vec![color];
        self.add(text);
    }

    fn add_with_mult(&mut self, text: &[u8], colors: Vec<(TermColor, bool)>) {
        self.new_color = colors;
        self.change_or_add(text);
    }

    fn push_with(&mut self, char: u8, color: (TermColor, bool)) {
        self.new_color = vec![color];
        self.push(char);
    }

    fn into_data(self) -> Vec<Highlight> {
        self.vec
    }
}

const BASE_COLOR: (TermColor, bool) = (TermColor::White, true);

impl LineRenderer {
    fn change_or_add(&mut self, str: &[u8]) {
        //if length 0, color shouldn't change either
        if str.len() == 0 {
            return;
        }
        // if color changed to base, nothing is needed
        if self.new_color != vec![BASE_COLOR] {
            // colors are same -- add to last
            if self.old_color == self.new_color {
                // should always be safe due to the not BASE_COLOR check
                let highlight = self.vec.last_mut().unwrap();
                highlight.length += str.len();
            // colors have changed
            } else {
                self.vec.push(Highlight {
                    line: self.curr_line,
                    index: self.curr_index,
                    length: str.len(),
                    color: self.new_color.iter().map(Self::get_color_str).collect(),
                });
            }
        }
        self.curr_index += str.len();
        self.old_color = self.new_color.clone();
    }
    fn get_color_str(color: &(TermColor, bool)) -> &'static str {
        match color {
            (TermColor::Black, false) => "black",
            (TermColor::Black, true) => "b_black",
            (TermColor::Red, false) => "red",
            (TermColor::Red, true) => "b_red",
            (TermColor::Green, false) => "green",
            (TermColor::Green, true) => "b_green",
            (TermColor::Yellow, false) => "yellow",
            (TermColor::Yellow, true) => "b_yellow",
            (TermColor::Blue, false) => "blue",
            (TermColor::Blue, true) => "b_blue",
            (TermColor::Purple, false) => "purple",
            (TermColor::Purple, true) => "b_purple",
            (TermColor::Cyan, false) => "cyan",
            (TermColor::Cyan, true) => "b_cyan",
            (TermColor::White, false) => "white",
            (TermColor::White, true) => "b_white",
        }
    }
}
