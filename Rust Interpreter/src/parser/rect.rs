use super::*;
use basic_func::BasicState;

#[derive(Debug)]

pub struct RectState {
    count: u8,
}

impl BasicState for RectState {
    fn get_name(&self) -> &'static str {
        "Rect"
    }

    fn do_first(&self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Rect {
                locs,
                indexes: [usize::MAX; 4],
                end: usize::MAX,
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize) {
        if let Expr::Rect { indexes, .. } = expr {
            indexes[self.count as usize] = index;
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> basic_func::CloseType {
        match self.count {
            0..=2 => basic_func::CloseType::Unable,
            3 => basic_func::CloseType::Able,
            4 => basic_func::CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn end(&mut self, expr: &mut Expr, index: usize) {
        if let Expr::Rect { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl RectState {
    pub fn new() -> Self {
        RectState { count: 0 }
    }
}
