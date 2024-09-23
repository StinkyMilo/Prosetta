use super::*;
use crate::parser::CloseType;
use basic_func::BasicState;

#[derive(Debug)]

pub struct ReturnState {
    count: u8,
}

impl BasicState for ReturnState {
    fn can_happen(&self, env: &mut Environment) -> bool {
        for parent in &mut *env.parents {
            if let Expr::Function { .. } = parent {
                return true;
            }
        }
        false
    }

    fn get_name(&self) -> &'static str {
        "Return"
    }

    fn do_first(&self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Return {
                locs,
                index: None,
                end: End::none(),
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, idx: usize) {
        if let Expr::Return { index, .. } = expr {
            *index = Some(idx);
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        match self.count {
            0 => CloseType::Able,
            1 => CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::Return { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl ReturnState {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}
