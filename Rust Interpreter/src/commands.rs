use std::ops::Index;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum OperatorType {
    Add,
    Sub,
    Mult,
    Div,
    Mod,
    Exp,
    Log,
}

#[derive(PartialEq, Debug)]
pub enum Prints {
    Var(usize),
    Word(String, usize),
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    NoneStat,
    NoneExpr,
    //stats
    Arc {
        locs: Vec<usize>,
        indexes: [usize; 4],
    },
    Line {
        locs: Vec<usize>,
        indexes: [usize; 4],
    },
    Set {
        locs: Vec<usize>,
        name_start: usize,
        name: Vec<u8>,
        value_index: usize,
    },
    Rect {
        locs: Vec<usize>,
        indexes: [usize; 4],
    },
    Print {
        locs: Vec<usize>,
        data: Vec<Prints>,
    },
    //expr
    Var {
        name_start: usize,
        name: Vec<u8>,
    },
    WordNum {
        locs: Vec<usize>,
        str_start: usize,
        str: Vec<u8>,
    },
    Operator {
        func_type: OperatorType,
        locs: Vec<usize>,
        indexes: Vec<usize>,
    },
    LitNum {
        locs: Vec<usize>,
        str_start: usize,
        str_length: usize,
        value: i64,
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
