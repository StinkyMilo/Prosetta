use super::*;
use crate::parser::CloseType;
use basic_func::BasicState;

#[derive(Debug)]
pub struct ListState {
    count: u8,
}
impl BasicState for ListState {
    fn get_name(&self) -> &'static str {
        "List"
    }

    fn get_state_return(&self) -> ReturnType {
        ReturnType::List
    }

    fn get_child_type(&self) -> Types {
        Types::Any
    }

    fn do_first(&self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::List {
                locs,
                indexes: Vec::new(),
                end: End::none(),
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize) {
        if let Expr::List { indexes, .. } = expr {
            indexes.push(index);
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        CloseType::Able
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::List { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl ListState {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}
