use basic_func::BasicState;

use super::*;

#[derive(Debug)]
pub struct RandState {
    count: u8,
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

    fn do_first(&self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Rand {
                locs,
                indexes: [usize::MAX; 2],
                end: End::none(),
            };
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize, _: ReturnType) {
        if let Expr::Rand { indexes, .. } = expr {
            indexes[self.count as usize] = index;
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    // this is a mess
    fn can_close(&self) -> CloseType {
        match self.count {
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
        Self { count: 0 }
    }
}
