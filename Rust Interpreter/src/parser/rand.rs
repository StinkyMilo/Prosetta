use basic_func::BasicState;

use super::*;

#[derive(Debug)]
pub struct RandState {
    next_child_index: u8,
}

impl BasicState for RandState {
    fn get_name(&self) -> &'static str {
        "Rand"
    }

    fn get_state_return(&self) -> ReturnType {
        ReturnType::Number
    }

    fn get_child_type(&self) -> Types {
        Types::Number
    }

    fn do_first(&mut self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.next_child_index == 0;
        if ret {
            *expr = Expr::Rand {
                locs,
                indexes: [usize::MAX; 2],
                end: End::none(),
            };
            self.next_child_index += 1;
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize, _: ReturnType) {
        if let Expr::Rand { indexes, .. } = expr {
            indexes[self.next_child_index as usize] = index;
            self.next_child_index += 1;
        } else {
            unreachable!()
        }
    }

    // this is a mess
    fn can_close(&self) -> CloseType {
        match self.next_child_index {
            0..=1 => CloseType::Able,
            2 => CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::Rand { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl RandState {
    pub fn new() -> Self {
        Self { next_child_index: 0 }
    }
}
