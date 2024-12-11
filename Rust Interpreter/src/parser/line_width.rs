use super::*;
use crate::parser::CloseType;
use basic_func::BasicState;

#[derive(Debug)]

pub struct LineWidthState {
    count: u8,
}

impl BasicState for LineWidthState {
    fn get_name(&self) -> &'static str {
        "Line Width"
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
            *expr = Expr::LineWidth {
                locs,
                child_index: usize::MAX,
                end: End::none(),
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize, _: ReturnType) {
        if let Expr::LineWidth { child_index, .. } = expr {
            *child_index = index;
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
        if let Expr::LineWidth { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl LineWidthState {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}
