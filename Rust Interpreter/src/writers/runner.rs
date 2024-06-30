use crate::commands::*;

pub enum ValueType {
    // type , index
    Error(ErrorData),
    Number(i64),
    Float(f64),
}

impl ValueType{
    pub fn get_i64(&self) -> Option<i64> {
        match &self{
            ValueType::Error(_) => None,
            ValueType::Number(i) => Some(*i),
            ValueType::Float(_) => None,
        }
    }

    pub fn get_f64(&self) -> Option<f64> {
        match &self{
            ValueType::Error(_) => None,
            ValueType::Number(i) => Some(*i as f64),
            ValueType::Float(i) => Some(*i),
        }
    }
}


pub enum Error {
    Malformed,
    NotParsed,
    Div0,
}

pub struct ErrorData {
    error: Error,
    index: usize,
}

pub struct Bounds {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

pub enum Statement {
    Circle([i64; 3]),
    Ellipse([i64; 4]),
    Line([i64; 4]),
    Assign(Vec<u8>, ValueType),
    Square([i64; 3]),
    Rect([i64; 4]),
    Print(Vec<Prints>),
    Error(ErrorData),
}

pub struct ProgramRunner {
    pub bounds: Option<Bounds>,
    pub statements: Vec<Statement>,
}

impl ProgramRunner {
    pub fn new() -> Self {
        Self {
            bounds: None,
            statements: Vec::new(),
        }
    }

    pub fn run(&mut self, exprs: &ExprArena, line_starts: &[usize]) {
        for statement in line_starts {
            run_statement(exprs, *statement,0);
        }
    }
}

impl ProgramRunner {
    fn add_to_bounds(&mut self, point: (i64, i64)) {
        if let Some(bounds) = &mut self.bounds {
            *bounds = Bounds {
                x1: bounds.x1.min(point.0),
                y1: bounds.y1.min(point.1),
                x2: bounds.x2.max(point.0),
                y2: bounds.y2.max(point.1),
            };
        } else {
            self.bounds = Some(Bounds {
                x1: point.0,
                y1: point.1,
                x2: point.0,
                y2: point.1,
            });
        }
    }


}
fn run_statement(
    exprs: &ExprArena,
    index: usize,
    mut global_index: usize,
) -> (Statement, usize) {
    match exprs[index] {
        Expr::NoneStat | Expr::NoneExpr => (
            Statement::Error(ErrorData {
                error: Error::NotParsed,
                index: global_index,
            }),
            global_index,
        ),
        Expr::Arc { locs, indexes ,end} => {
            let data;
            (data,global_index) = run_3_or_4_args(exprs,indexes,global_index);
            let ret = match data {
                Ok(([a,b,c,d],has_last)) =>{
                    if has_last {
                        Statement::Ellipse([a,b,c,d])
                    } else {
                        Statement::Circle([a,b,c])
                    }
                }
                Err(data)=>{
                    Statement::Error(data)
                }
            };

            (
                ret,
                global_index,
            )
        }
        Expr::Line { locs, indexes,end } => todo!(),
        Expr::Assign {
            locs,
            name_start,
            name,
            value_index,
            end,
        } => todo!(),
        Expr::Rect { locs, indexes,end } => {
        let data;
        (data,global_index) = run_3_or_4_args(exprs,indexes,global_index);
        let ret = match data {
            Ok(([a,b,c,d],has_last)) =>{
                if has_last {
                    Statement::Rect([a,b,c,d])
                } else {
                    Statement::Square([a,b,c])
                }
            }
            Err(data)=>{
                Statement::Error(data)
            }
        };

        (
            ret,
            global_index,
        )
    }
        Expr::Print { locs, data ,end} => todo!(),
        expr => (
            Statement::Error(ErrorData {
                error: Error::NotParsed,
                index: global_index + 1,
            }),
            global_index,
        ),
    }
}
fn run_expr(exprs: &ExprArena, index: usize) -> ValueType {
    ValueType::Number(8)
}

fn run_3_or_4_args(exprs: &ExprArena, indexes: [usize; 4], mut global_index: usize,)-> (Result<([i64; 4],bool),ErrorData>, usize) {
    let data = [0i64; 4];
    let has_last = false;

    // first 3
    for j in 0..3 {
        if indexes[j] == usize::MAX {
            return (Err(ErrorData {
                error: Error::NotParsed,
                index: global_index,
            }),global_index);
        }
        if self.write_expr(exprs) {

        }
    }
        match (j==3,indexes[j] == usize::MAX){
            (true, true) => {},
            (true, false) => {has_last = false;},
            (false, true) => { return (Statement::Error(ErrorData {
                error: Error::NotParsed,
                index: global_index,
            }),global_index);
        },
            (false, false) => todo!(),
        }
        // if indexes[j] == usize::MAX {
        //     if j!=3{
        //         return Statement::Error(ErrorData {
        //             error: Error::NotParsed,
        //             index: global_index,
        //         }),
        //     }
        // }

    (Ok((data,has_last)),global_index)
    }

