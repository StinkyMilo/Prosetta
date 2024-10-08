use super::*;
use crate::parser::CloseType;
use basic_func::BasicState;

#[derive(Debug)]
pub struct MoveToState {
    count: u8,
}
impl BasicState for MoveToState {
    fn get_name(&self) -> &'static str {
        "MoveTo"
    }

    fn get_type(&self) -> StateType {
        StateType::Stat
    }

    fn do_first(&self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::MoveTo {
                locs,
                indexes: [usize::MAX; 2],
                end: End::none(),
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize) {
        if let Expr::MoveTo { indexes, .. } = expr {
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
            2 => CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::MoveTo { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl MoveToState {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}
