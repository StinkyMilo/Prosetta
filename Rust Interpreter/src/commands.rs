use std::ops::Index;

use crate::parser::End;
use crate::parser::multi_lit_num::VarOrInt;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum OperatorType {
    Add,
    Sub,
    Mult,
    Div,
    Mod,
    Exp,
    Log,
    LessThan,
    GreaterThan,
    And,
    Or,
    Equals,
    Not,
}

#[derive(PartialEq, Debug)]
pub enum Prints {
    /// child_index
    Var(usize),
    /// child_index
    String(usize),
    /// value, string_index
    Word(Vec<u8>, usize),
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    NoneStat,
    NoneExpr,
    //stats
    Arc {
        locs: Vec<usize>,
        indexes: [usize; 4],
        end: End,
    },
    Line {
        locs: Vec<usize>,
        indexes: [usize; 4],
        end: End,
    },
    Rect {
        locs: Vec<usize>,
        indexes: [usize; 4],
        end: End,
    },
    Assign {
        locs: Vec<usize>,
        name_start: usize,
        name: Vec<u8>,
        value_index: usize,
        end: End,
    },
    Print {
        locs: Vec<usize>,
        data: Vec<Prints>,
        end: End,
    },
    If {
        locs: Vec<usize>,
        indexes: Vec<usize>,
        else_index: usize,
        end: End,
    },
    While {
        locs: Vec<usize>,
        indexes: Vec<usize>,
        end: End,
    },
    Else {
        locs: Vec<usize>,
        indexes: Vec<usize>,
        end: End,
    },
    //expr
    Var {
        name_start: usize,
        name: Vec<u8>,
    },
    WordNum {
        locs: Vec<usize>,
        str_start: usize,
        str_len: usize,
        end: End,
    },
    Operator {
        locs: Vec<usize>,
        func_type: OperatorType,
        indexes: Vec<usize>,
        end: End,
    },
    LitNum {
        str_start: usize,
        str_length: usize,
        value: i64,
    },
    //TODO: Could make a compiler optimization to pre-evaluate this if there are no variables
    MultiLitNum {
        str_start: usize,
        locs: Vec<usize>,
        values: Vec<VarOrInt>,
        single_value: Option<i64>,
        end: End,
    },
    Skip {
        locs: Vec<usize>,
        index: usize,
        start: usize,
        end: End,
    },
    Color {
        locs: Vec<usize>,
        indexes: [usize; 3],
        end: End,
    },
    Fill {
        locs: Vec<usize>,
        indexes: [usize; 3],
        end: End,
    },
    Stroke {
        locs: Vec<usize>,
        indexes: [usize; 3],
        end: End,
    },
    LitCol {
        str_start: usize,
        str_length: usize,
        value: Vec<u8>,
    },
    LitString {
        str_start: usize,
        str: Vec<u8>,
    },
}

impl Expr {
    pub fn is_none(&self) -> bool {
        match self {
            Expr::NoneStat => true,
            Expr::NoneExpr => true,
            _ => false,
        }
    }
    // pub fn is_stat(&self) -> bool {
    //     match self {
    //         Expr::Arc { .. } | Expr::Line { .. } | Expr::Rect { .. } => true,
    //         Expr::Assign { .. } | Expr::Print { .. } => true,
    //         Expr::If { .. } | Expr::Else { .. } | Expr::While { .. } => true,
    //         _ => false,
    //     }
    // }
}

#[derive(Debug)]
pub struct ExprArena {
    pub vec: Vec<Expr>,
}
impl Index<usize> for ExprArena {
    type Output = Expr;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.vec.len() {
            &self.vec[index]
        } else {
            &Expr::NoneExpr
        }
    }
}
