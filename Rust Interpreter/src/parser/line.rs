use super::*;
use crate::parser::CloseType;
use basic_func::BasicState;

#[derive(Debug)]
pub struct LineState {
    count: u8,
}
impl BasicState for LineState {
    fn get_name(&self) -> &'static str {
        "Line"
    }

    fn do_first(&self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Line {
                locs,
                indexes: [usize::MAX; 4],
                end: usize::MAX,
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize) {
        if let Expr::Line { indexes, .. } = expr {
            indexes[self.count as usize] = index;
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        match self.count {
            0..=3 => CloseType::Unable,
            4 => CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn end(&mut self, expr: &mut Expr, index: usize) {
        if let Expr::Line { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl LineState {
    pub fn new() -> Self {
        LineState { count: 0 }
    }
}
