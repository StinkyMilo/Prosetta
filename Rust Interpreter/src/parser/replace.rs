use super::*;
use crate::parser::CloseType;
use basic_func::BasicState;

#[derive(Debug)]
pub struct ReplaceState {
    count: u8,
}
impl BasicState for ReplaceState {
    fn get_name(&self) -> &'static str {
        "Replace"
    }

    fn get_state_return(&self) -> ReturnType {
        ReturnType::Void
    }

    fn get_child_type(&self) -> Types {
        match self.count {
            0 => Types::List,
            1 => Types::Number,
            2 => Types::Any,
            _ => unreachable!(),
        }
    }

    fn do_first(&self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Replace {
                locs,
                indexes: [usize::MAX; 3],
                end: End::none(),
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize, _: ReturnType) {
        if let Expr::Replace { indexes, .. } = expr {
            indexes[self.count as usize] = index;
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        match self.count {
            0..=2 => CloseType::Unable,
            3 => CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::Replace { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl ReplaceState {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}
