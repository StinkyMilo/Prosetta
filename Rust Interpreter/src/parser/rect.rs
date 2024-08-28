use super::*;
use crate::parser::CloseType;
use basic_func::BasicState;

#[derive(Debug)]

pub struct RectState {
    count: u8,
}

impl BasicState for RectState {
    fn get_name(&self) -> &'static str {
        "Rect"
    }

    fn do_first(&self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Rect {
                locs,
                indexes: [usize::MAX; 4],
                end: usize::MAX,
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize) {
        if let Expr::Rect { indexes, .. } = expr {
            indexes[self.count as usize] = index;
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        match self.count {
            0..=2 => CloseType::Unable,
            3 => CloseType::Able,
            4 => CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: usize) {
        if let Expr::Rect { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl RectState {
    pub fn new() -> Self {
        RectState { count: 0 }
    }
}
