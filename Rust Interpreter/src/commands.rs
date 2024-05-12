use std::ops::Index;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BiFunctionType {
    Add,
    Sub,
    Mult,
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    NoneStat,
    NoneExpr,
    //stats
    Eq {
        locs: Vec<usize>,
        name_start: usize,
        name: Vec<u8>,
        value_index: usize,
    },
    Line {
        locs: Vec<usize>,
        indexes:[usize;4],
    },
    Circle {
        locs: Vec<usize>,
        indexes:[usize;3],
    },

    //expr
    Var {
        name_start: usize,
        name: Vec<u8>,
    },
    Num {
        locs: Vec<usize>,
        str_start: usize,
        str: Vec<u8>,
    },
    BiFunction {
        func_type: BiFunctionType,
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
    // pub fn get_name(&self) -> &'static str {
    //     match self {
    //         Expr::NoneStat => "NoneStat",
    //         Expr::NoneExpr => "NoneExpr",
    //         Expr::Eq { .. } => "Equals",
    //         Expr::Line { .. } => "Line",
    //         Expr::Circle { .. } => "Circle",
    //         Expr::Var { .. } => "Var",
    //         Expr::Num { .. } => "Num",
    //         Expr::BiFunction { func_type, .. } => match func_type {
    //             BiFunctionType::Add => "Add",
    //             BiFunctionType::Sub => "Sub",
    //             BiFunctionType::Mult => "Mult",
    //         },
    //         Expr::LitNum { .. } => "LitNum",
    //     }
    // }
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
