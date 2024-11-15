use super::*;
use crate::parser::CloseType;
use basic_func::BasicState;

#[derive(Debug)]
pub struct AppendState {
    count: u8,
}

impl BasicState for AppendState {
    fn get_name(&self) -> &'static str {
        "Append"
    }

    fn get_state_return(&self) -> ReturnType {
        ReturnType::Void
    }

    fn get_child_type(&self) -> Types {
        match self.count {
            0 => Types::List,
            1 => Types::Any,
            2 => Types::Number,
            _ => unreachable!(),
        }
    }

    fn do_first(&self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Append {
                locs,
                indexes: [usize::MAX; 3],
                end: End::none(),
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize, _: ReturnType) {
        if let Expr::Append { indexes, .. } = expr {
            indexes[self.count as usize] = index;
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        match self.count {
            0..=1 => CloseType::Unable,
            2 => CloseType::Able,
            3 => CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::Append { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl AppendState {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}
