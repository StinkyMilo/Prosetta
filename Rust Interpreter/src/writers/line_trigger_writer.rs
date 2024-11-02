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

use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[allow(dead_code)]
#[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct Highlight {
    pub line: usize,
    pub index: usize,
    pub length: usize,
    pub color: Vec<String>,
}

// #[cfg(feature = "wasm")]
// use serde_wasm_bindgen::to_value;
// #[cfg(feature = "wasm")]
// use wasm_bindgen::JsValue;
// #[cfg(feature = "wasm")]
// #[allow(dead_code)]
// #[wasm_bindgen]
// impl Highlight {
//     pub fn get_colors(&self) -> JsValue {
//         serde_wasm_bindgen::to_value(&self.color).unwrap()
//     }
// }

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
    fn get_color_str(color: &(TermColor, bool)) -> String {
        match color {
            (TermColor::Black, false) => "term_black",
            (TermColor::Black, true) => "term_b_black",
            (TermColor::Red, false) => "term_red",
            (TermColor::Red, true) => "term_b_red",
            (TermColor::Green, false) => "term_green",
            (TermColor::Green, true) => "term_b_green",
            (TermColor::Yellow, false) => "term_yellow",
            (TermColor::Yellow, true) => "term_b_yellow",
            (TermColor::Blue, false) => "term_blue",
            (TermColor::Blue, true) => "term_b_blue",
            (TermColor::Purple, false) => "term_purple",
            (TermColor::Purple, true) => "term_b_purple",
            (TermColor::Cyan, false) => "term_cyan",
            (TermColor::Cyan, true) => "term_b_cyan",
            (TermColor::White, false) => "term_white",
            (TermColor::White, true) => "term_b_white",
        }
        .to_string()
    }
}
