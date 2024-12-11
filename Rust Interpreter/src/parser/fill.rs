use super::*;
use crate::parser::CloseType;
use basic_func::BasicState;

#[derive(Debug)]

pub struct FillState {
    count: u8,
    got_color: bool,
}

impl BasicState for FillState {
    fn get_name(&self) -> &'static str {
        "Fill"
    }

    fn get_state_return(&self) -> ReturnType {
        ReturnType::Void
    }

    fn get_child_type(&self) -> Types {
        match self.count {
            0 => Types::Color | Types::Number,
            _ => Types::Number,
        }
    }

    fn do_first(&mut self, expr: &mut Expr, locs: Vec<usize>) -> bool {
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

    fn add_child(&mut self, expr: &mut Expr, index: usize, return_type: ReturnType) {
        if return_type == ReturnType::Color {
            self.got_color = true;
        }
        if let Expr::Fill { indexes, .. } = expr {
            indexes[self.count as usize] = index;
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        let can_at_1 = if self.got_color {
            CloseType::Force
        } else {
            CloseType::Able
        };
        match self.count {
            0 => CloseType::Unable,
            1 => can_at_1,
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
        Self {
            count: 0,
            got_color: false,
        }
    }
}
