use super::*;
use crate::parser::CloseType;
use basic_func::BasicState;

#[derive(Debug)]
pub struct BezierState {
    count: u8,
}
impl BasicState for BezierState {
    fn get_name(&self) -> &'static str {
        "Bezier"
    }

    fn get_state_return(&self) -> ReturnType {
        ReturnType::Void
    }

    fn get_child_type(&self) -> Types {
        Types::Number
    }

    fn do_first(&self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Bezier {
                locs,
                indexes: Vec::new(),
                end: End::none(),
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize) {
        if let Expr::Bezier { indexes, .. } = expr {
            indexes.push(index);
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        if self.count < 4 {
            return CloseType::Unable;
        }
        if self.count % 2 == 0 {
            return CloseType::Able;
        }
        return CloseType::Unable;
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::Bezier { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl BezierState {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}
