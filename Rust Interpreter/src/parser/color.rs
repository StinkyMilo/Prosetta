use super::*;
use crate::parser::CloseType;
use basic_func::BasicState;

#[derive(Debug)]

pub struct ColorState {
    count: u8
}

impl BasicState for ColorState {
    fn get_name(&self) -> &'static str {
        "Color"
    }

    fn do_first(&self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Color { 
                locs,
                indexes: [usize::MAX; 3], 
                end: usize::MAX
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize) {
        if let Expr::Color {indexes, ..} = expr {
            indexes[self.count as usize] = index;
            self.count += 1;
        }else{
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        match self.count {
            0..=2 => CloseType::Unable,
            3 => CloseType::Force,
            _ => unreachable!()
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: usize) {
        if let Expr::Color {end, ..} = expr  {
            *end = index;
        }else{
            unreachable!()
        }
    }
}

impl ColorState{
    pub fn new () -> Self {
        Self { count: 0 }
    }
}