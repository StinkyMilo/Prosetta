use std::ops::Index;


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
        x_index: usize,
        y_index: usize,
        x2_index: usize,
        y2_index: usize,
    },
    Circle {
        locs: Vec<usize>,
        x_index: usize,
        y_index: usize,
        r_index: usize,
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
    Mult {
        locs: Vec<usize>,
        a_index: usize,
        b_index: usize,
    },
    Add {
        locs: Vec<usize>,
        a_index: usize,
        b_index: usize,
    },
}

impl Expr{
    pub fn get_name(&self)->&'static str{
        match self{
            Expr::NoneStat => "NoneStat",
            Expr::NoneExpr => "NoneExpr",
            Expr::Eq {..} => "Equals",
            Expr::Line {.. } => "Line",
            Expr::Circle {..} => "Circle",
            Expr::Var { ..} =>"Var",
            Expr::Num { .. } => "Num",
            Expr::Mult { .. } => "Mult",
            Expr::Add { .. } => "Add",
        }
    }
    pub fn is_none(&self)->bool{
        match self{
            Expr::NoneStat => true,
            Expr::NoneExpr => true,
            _=>false
        }
    }
}

#[derive(Debug)]
pub struct ExprArena{
    pub vec:Vec<Expr>
}
impl Index<usize> for ExprArena {
    type Output = Expr;

    fn index(&self, index:usize) -> &Self::Output { 
        if index<self.vec.len(){
            &self.vec[index]
        }else{
            &Expr::NoneExpr
        }
    }
}