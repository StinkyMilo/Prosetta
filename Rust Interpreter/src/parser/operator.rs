use basic_func::BasicState;

use super::*;

#[derive(Debug)]
pub struct OperatorState {
    fn_type: OperatorType,
    count: u8,
}

impl BasicState for OperatorState {
    fn get_name(&self) -> &'static str {
        match self.fn_type {
            OperatorType::Add => "Add",
            OperatorType::Mult => "Mult",
            OperatorType::Sub => "Sub",
            OperatorType::Div => "Div",
            OperatorType::Mod => "Mod",
            OperatorType::Exp => "Exp",
            OperatorType::Log => "Log",
        }
    }

    fn do_first(&self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Operator {
                locs,
                func_type: self.fn_type,
                indexes: Vec::new(),
                end: usize::MAX,
            };
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize) {
        if let Expr::Operator { indexes, .. } = expr {
            indexes.push(index);
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    // this is a mess
    fn can_close(&self) -> CloseType {
        match (self.fn_type, self.count) {
            (OperatorType::Add, 2..) => CloseType::Able,
            (OperatorType::Sub, 1) => CloseType::Able,
            (OperatorType::Sub, 2) => CloseType::Force,
            (OperatorType::Mult, 2..) => CloseType::Able,
            (OperatorType::Div, 2) => CloseType::Force,
            (OperatorType::Mod, 2) => CloseType::Force,
            (OperatorType::Exp, 1) => CloseType::Able,
            (OperatorType::Exp, 2) => CloseType::Force,
            (OperatorType::Log, 1) => CloseType::Able,
            (OperatorType::Log, 2) => CloseType::Force,
            _ => CloseType::Unable,
        }
    }

    fn end(&mut self, expr: &mut Expr, index: usize) {
        if let Expr::Operator { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl OperatorState {
    fn new(fn_type: OperatorType) -> Self {
        OperatorState { fn_type, count: 0 }
    }

    pub fn new_add() -> Self {
        Self::new(OperatorType::Add)
    }

    pub fn new_sub() -> Self {
        Self::new(OperatorType::Sub)
    }

    pub fn new_mult() -> Self {
        Self::new(OperatorType::Mult)
    }

    pub fn new_div() -> Self {
        Self::new(OperatorType::Div)
    }

    pub fn new_mod() -> Self {
        Self::new(OperatorType::Mod)
    }

    pub fn new_exp() -> Self {
        Self::new(OperatorType::Exp)
    }

    pub fn new_log() -> Self {
        Self::new(OperatorType::Log)
    }
}