use super::*;
use crate::parser::CloseType;
use basic_func::BasicState;

#[derive(Debug)]

pub struct RotateState {
    count: u8,
}

impl BasicState for RotateState {
    fn get_name(&self) -> &'static str {
        "Rotate"
    }

    fn do_first(&self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Rotate {
                locs,
                index: usize::MAX,
                end: End::none(),
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, idx: usize) {
        if let Expr::Rotate { index, .. } = expr {
            *index = idx;
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        match self.count {
            0 => CloseType::Unable,
            1 => CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::Rotate { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl RotateState {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}
