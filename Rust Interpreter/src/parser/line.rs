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

    fn get_state_return(&self) -> ReturnType {
        ReturnType::Void
    }

    fn get_child_type(&self) -> Types {
        Types::Number
    }

    fn do_first(&mut self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Line {
                locs,
                indexes: [usize::MAX; 4],
                end: End::none(),
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize, _: ReturnType) {
        if let Expr::Line { indexes, .. } = expr {
            indexes[self.count as usize] = index;
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        match self.count {
            0 => CloseType::Unable,
            1 => CloseType::Able,
            2 => CloseType::Able,
            3 => CloseType::Able,
            4 => CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::Line { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl LineState {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}
