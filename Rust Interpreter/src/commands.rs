use std::ops::Index;

use crate::parser::multi_lit_num::VarOrInt;
use crate::parser::string_lit::VarOrStr;
use crate::parser::{End, SubStrData, Title};

use bitflags::bitflags;

bitflags! {
    #[derive(Debug,Clone,Copy,Hash,PartialEq,Eq)]
    pub struct Types: u32 {
        const Void =   0;
        const Number = 0b1;
        const Bool =   0b10;
        const Booly =  0b11;
        const String = 0b100;
        const Color =  0b1000;
        const List =   0b10000;
        const Any =    0b11111;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReturnType {
    Void,
    Number,
    Bool,
    String,
    Color,
    List,
    Any,
}

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
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TrigType {
    Sin,
    Cos,
    Tan,
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    NoneStat,
    NoneExpr,
    //stats
    Title {
        data: Title,
    },
    Arc {
        locs: Vec<usize>,
        indexes: [usize; 4],
        end: End,
    },
    Bezier {
        locs: Vec<usize>,
        indexes: Vec<usize>,
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
        var: SubStrData,
        first: bool,
        value_index: usize,
        end: End,
    },
    Print {
        locs: Vec<usize>,
        indexes: Vec<usize>,
        single_word: Option<Vec<u8>>,
        single_word_start: usize,
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
        var: SubStrData,
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
        value_positions: Vec<(usize, usize)>,
        single_value: Option<i64>,
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
        str_end: usize,
        str: Vec<VarOrStr>,
    },
    MoveTo {
        locs: Vec<usize>,
        indexes: [usize; 2],
        end: End,
    },
    LineWidth {
        locs: Vec<usize>,
        child_index: usize,
        end: End,
    },
    Rotate {
        locs: Vec<usize>,
        index: usize,
        end: End,
    },
    Function {
        locs: Vec<usize>,
        func: SubStrData,
        args: Vec<SubStrData>,
        indexes: Vec<usize>,
        end: End,
    },
    FunctionCall {
        locs: Vec<usize>,
        func: SubStrData,
        indexes: Vec<usize>,
        end: End,
    },
    Return {
        locs: Vec<usize>,
        index: Option<usize>,
        end: End,
    },
    Append {
        locs: Vec<usize>,
        indexes: [usize; 3],
        end: End,
    },
    Delete {
        locs: Vec<usize>,
        indexes: [usize; 2],
        end: End,
    },
    Replace {
        locs: Vec<usize>,
        indexes: [usize; 3],
        end: End,
    },
    Find {
        locs: Vec<usize>,
        indexes: [usize; 2],
        end: End,
    },
    Index {
        locs: Vec<usize>,
        indexes: [usize; 2],
        end: End,
    },
    List {
        locs: Vec<usize>,
        indexes: Vec<usize>,
        end: End,
    },
    ForEach {
        var: SubStrData,
        locs: Vec<usize>,
        indexes: Vec<usize>,
        end: End,
    },
    Length {
        locs: Vec<usize>,
        index: usize,
        end: End,
    },
    Not {
        locs: Vec<usize>,
        word: Vec<u8>,
        str_start: usize,
        str_len: usize,
        end: End,
    },
    Frame {
        locs: Vec<usize>,
    },
    Comment {
        start: usize,
        end: usize,
        comment: Vec<u8>,
    },
    Trig {
        locs: Vec<usize>,
        func_type: TrigType,
        index: usize,
        end: End,
    },
    Rand {
        locs: Vec<usize>,
        indexes: [usize; 2],
        end: End,
    },
    Floor {
        locs: Vec<usize>,
        index: usize,
        end: End,
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
