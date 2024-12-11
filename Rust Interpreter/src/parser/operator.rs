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
            OperatorType::Sub => "Sub",
            OperatorType::Mult => "Mult",
            OperatorType::Div => "Div",
            OperatorType::Mod => "Mod",
            OperatorType::Exp => "Exp",
            OperatorType::Log => "Log",
            OperatorType::LessThan => "LessThan",
            OperatorType::GreaterThan => "GreaterThan",
            OperatorType::And => "And",
            OperatorType::Or => "Or",
            OperatorType::Equals => "Equals",
            OperatorType::Not => "Not",
        }
    }

    fn get_state_return(&self) -> ReturnType {
        match self.fn_type {
            OperatorType::Add
            | OperatorType::Sub
            | OperatorType::Mult
            | OperatorType::Div
            | OperatorType::Mod
            | OperatorType::Exp
            | OperatorType::Log => ReturnType::Number,
            OperatorType::LessThan | OperatorType::GreaterThan => ReturnType::Bool,
            OperatorType::And | OperatorType::Or | OperatorType::Not => ReturnType::Number,
            OperatorType::Equals => ReturnType::Any,
        }
    }

    fn get_child_type(&self) -> Types {
        match self.fn_type {
            OperatorType::Add
            | OperatorType::Sub
            | OperatorType::Mult
            | OperatorType::Div
            | OperatorType::Mod
            | OperatorType::Exp
            | OperatorType::Log => Types::Number,
            OperatorType::LessThan | OperatorType::GreaterThan => Types::Number,
            OperatorType::And | OperatorType::Or | OperatorType::Not => Types::Number | Types::Bool,
            OperatorType::Equals => Types::Any,
        }
    }

    fn do_first(&mut self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Operator {
                locs,
                func_type: self.fn_type,
                indexes: Vec::new(),
                end: End::none(),
            };
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize, _: ReturnType) {
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
            (OperatorType::LessThan, 2) => CloseType::Force,
            (OperatorType::GreaterThan, 2) => CloseType::Force,
            (OperatorType::And, 2..) => CloseType::Able,
            (OperatorType::Or, 2..) => CloseType::Able,
            (OperatorType::Equals, 2..) => CloseType::Able,
            (OperatorType::Not, 1) => CloseType::Force,
            _ => CloseType::Unable,
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::Operator { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl OperatorState {
    fn new(fn_type: OperatorType) -> Self {
        Self { fn_type, count: 0 }
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

    pub fn new_less_than() -> Self {
        Self::new(OperatorType::LessThan)
    }

    pub fn new_greater_than() -> Self {
        Self::new(OperatorType::GreaterThan)
    }

    pub fn new_and() -> Self {
        Self::new(OperatorType::And)
    }

    pub fn new_or() -> Self {
        Self::new(OperatorType::Or)
    }

    pub fn new_equals() -> Self {
        Self::new(OperatorType::Equals)
    }

    pub fn new_not() -> Self {
        Self::new(OperatorType::Not)
    }
}
