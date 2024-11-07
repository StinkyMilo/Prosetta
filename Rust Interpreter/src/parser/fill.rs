use super::*;
use crate::parser::CloseType;
use basic_func::BasicState;

#[derive(Debug)]

pub struct FillState {
    count: u8,
}

impl BasicState for FillState {
    fn get_name(&self) -> &'static str {
        "Fill"
    }

    fn get_state_type(&self) -> StateType {
        StateType::Stat
    }

    fn get_child_type(&self) -> Types {
        match self.count {
            0 => Types::Color | Types::Number,
            1..2 => Types::Number,
            _ => unreachable!(),
        }
    }

    fn do_first(&self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Fill {
                locs,
                indexes: [usize::MAX; 3],
                end: End::none(),
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize) {
        if let Expr::Fill { indexes, .. } = expr {
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
            2 => CloseType::Unable,
            3 => CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::Fill { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl FillState {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}
