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
    // child_index
    Var(usize),
    // value, string_index
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
        end: usize,
    },
    Line {
        locs: Vec<usize>,
        indexes: [usize; 4],
        end: usize,
    },
    Assign {
        locs: Vec<usize>,
        name_start: usize,
        name: Vec<u8>,
        value_index: usize,
        end: usize,
    },
    Rect {
        locs: Vec<usize>,
        indexes: [usize; 4],
        end: usize,
    },
    Print {
        locs: Vec<usize>,
        data: Vec<Prints>,
        end: usize,
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
        end: usize,
    },
    Operator {
        locs: Vec<usize>,
        func_type: OperatorType,
        indexes: Vec<usize>,
        end: usize,
    },
    LitNum {
        str_start: usize,
        str_length: usize,
        value: i64,
    },
    MultiLitNum {
        locs: Vec<usize>,
        num_indexes: Vec<usize>,
        end: usize,
    },
    Skip {
        locs: Vec<usize>,
        index: usize,
        start: usize,
        end: usize,
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
