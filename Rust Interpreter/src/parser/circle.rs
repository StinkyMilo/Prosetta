use super::*;
use crate::parser::CloseType;
use basic_func::BasicState;

#[derive(Debug)]
pub struct CircleState {
    count: u8,
}
impl BasicState for CircleState {
    fn get_name(&self) -> &'static str {
        "Circle"
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
            *expr = Expr::Arc {
                locs,
                indexes: [usize::MAX; 4],
                end: End::none(),
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize) {
        if let Expr::Arc { indexes, .. } = expr {
            indexes[self.count as usize] = index;
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        match self.count {
            0 => CloseType::Unable,
            1..=3 => CloseType::Able,
            /*
                1-argument: diameter of circle from current position
                2-argument: width and height of ellipse from current position
                3-argument: x and y coordinates then diameter of circle
                4-argument: x and y coordinate then width and height of ellipse
                Draw from the middle
            */
            4 => CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::Arc { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl CircleState {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}
