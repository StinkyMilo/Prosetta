use std::{collections::HashMap, default};

use crate::{commands::*, parser::State};

pub enum ValueType {
    // type , index
    Error(ErrorData),
    Number(i64),
    Float(f64),
}

impl ValueType {
    pub fn get_i64(&self) -> Option<i64> {
        match &self {
            ValueType::Error(_) => None,
            ValueType::Number(i) => Some(*i),
            ValueType::Float(_) => None,
        }
    }

    pub fn get_f64(&self) -> Option<f64> {
        match &self {
            ValueType::Error(_) => None,
            ValueType::Number(i) => Some(*i as f64),
            ValueType::Float(i) => Some(*i),
        }
    }

    // pub fn get_i64_cast(&self) -> Option<i64> {
    //     match &self {
    //         ValueType::Error(_) => None,
    //         ValueType::Number(i) => Some(*i),
    //         ValueType::Float(i) => f64_to_i64(*i),
    //     }
    // }
}

// fn f64_to_i64(val:f64)->Option<i64>{
//     if val.is_nan() || val>i64::MAX as f64||val<i64::MIN  as f64{
//         None
//     }else {
//         val.round().
//     }
// }
#[derive(Debug)]

pub enum Error {
    Malformed,
    NotParsed,
    Div0,
    Log,
    TypeError,
    NoVarible,
}

pub struct ErrorData {
    error: Error,
}

pub struct Bounds {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

pub enum Statement {
    Error(ErrorData, usize),
    Circle([f64; 3]),
    Ellipse([f64; 4]),
    Line([f64; 4]),
    Assign(Vec<u8>, ValueType),
    Square([f64; 3]),
    Rect([f64; 4]),
    Print(Vec<Prints>),
}
// name, index
type Vars = HashMap<Vec<u8>, ValueType>;
pub struct ProgramRunner {
    pub bounds: Option<Bounds>,
    pub vars: Vars,
    pub statements: Vec<Statement>,
}

#[allow(dead_code)]
impl ProgramRunner {
    pub fn new() -> Self {
        Self {
            bounds: None,
            vars: Vars::new(),
            statements: Vec::new(),
        }
    }

    pub fn run(&mut self, exprs: &ExprArena, line_starts: &[usize]) {
        let mut vars = Vars::new();
        let mut global_index = 0;
        for statement in line_starts {
            let stat;
            (stat, global_index) = run_statement(exprs, &vars, *statement, global_index);
            self.run_statement_bounds(stat);
        }
    }
}

impl ProgramRunner {
    fn run_statement_bounds(&mut self, stat: Statement) {
        match stat {
            Statement::Error(error, index) => {
                println!("{:?} Error occured at {}", error.error, index);
            }
            Statement::Assign(name, data) => {
                self.vars.insert(name, data);
            }
            s @ Statement::Circle([x, y, r]) => {
                self.add_to_bounds((x - r, y));
                self.add_to_bounds((x + r, y));
                self.add_to_bounds((x, y - r));
                self.add_to_bounds((x, y + r));
                self.statements.push(s);
            }
            s @ Statement::Ellipse([x, y, w, h]) => {
                self.add_to_bounds((x - w, y));
                self.add_to_bounds((x + w, y));
                self.add_to_bounds((x, y - h));
                self.add_to_bounds((x, y + h));
                self.statements.push(s)
            }
            s @ Statement::Line([x, y, x2, y2]) => {
                self.add_to_bounds((x, y));
                self.add_to_bounds((x2, y2));
                self.statements.push(s)
            }
            s @ Statement::Square([x, y, w]) => {
                self.add_to_bounds((x, y));
                self.add_to_bounds((x + w, y));
                self.add_to_bounds((x, y + w));
                self.add_to_bounds((x + w, y + w));
                self.statements.push(s)
            }
            s @ Statement::Rect([x, y, w, h]) => {
                self.add_to_bounds((x, y));
                self.add_to_bounds((x + w, y));
                self.add_to_bounds((x, y + h));
                self.add_to_bounds((x + w, y + h));
                self.statements.push(s)
            }
            s @ Statement::Print(_) => self.statements.push(s),
        };
    }

    fn add_to_bounds(&mut self, point: (f64, f64)) {
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
    vars: &Vars,
    index: usize,
    mut global_index: usize,
) -> (Statement, usize) {
    match &exprs[index] {
        Expr::NoneStat | Expr::NoneExpr => (
            Statement::Error(
                ErrorData {
                    error: Error::NotParsed,
                },
                global_index,
            ),
            global_index,
        ),
        Expr::Arc { locs, indexes, end } => {
            let data;
            set_global_to_loc(&mut global_index, locs);
            (data, _) = run_variable_length_args(exprs, vars, indexes, global_index, |vec| {
                match (vec.len(), vec) {
                    (0..=2, ..) => None,
                    (3, [a, b, c]) => Some((Statement::Circle([*a, *b, *c]), false)),
                    (4, [a, b, c, d]) => Some((Statement::Ellipse([*a, *b, *c, *d]), true)),
                    _ => unreachable!(),
                }
            });
            set_global_to_end(&mut global_index, *end);
            (data, global_index)
        }
        Expr::Line { locs, indexes, end } => {
            let data;
            set_global_to_loc(&mut global_index, locs);
            (data, _) = run_variable_length_args(exprs, vars, indexes, global_index, |vec| {
                match (vec.len(), vec) {
                    (0..=3, ..) => None,
                    (4, [a, b, c, d]) => Some((Statement::Line([*a, *b, *c, *d]), true)),
                    _ => unreachable!(),
                }
            });
            set_global_to_end(&mut global_index, *end);
            (data, global_index)
        }
        Expr::Assign {
            locs,
            name_start,
            name,
            value_index,
            end,
        } => todo!(),
        Expr::Rect { locs, indexes, end } => {
            let data;
            set_global_to_loc(&mut global_index, locs);
            (data, _) = run_variable_length_args(exprs, vars, indexes, global_index, |vec| {
                match (vec.len(), vec) {
                    (0..=2, ..) => None,
                    (3, [a, b, c]) => Some((Statement::Square([*a, *b, *c]), false)),
                    (4, [a, b, c, d]) => Some((Statement::Rect([*a, *b, *c, *d]), true)),
                    _ => unreachable!(),
                }
            });
            set_global_to_end(&mut global_index, *end);
            (data, global_index)
        }
        Expr::Print { locs, data, end } => {
            set_global_to_loc(&mut global_index, locs);
            set_global_to_end(&mut global_index, *end);
            (data, global_index)
        }
        // this should never run - but if it does than error
        _ => (
            Statement::Error(
                ErrorData {
                    error: Error::Malformed,
                },
                global_index,
            ),
            global_index,
        ),
    }
}

fn run_expr(
    exprs: &ExprArena,
    vars: &Vars,
    index: usize,
    mut global_index: usize,
) -> (ValueType, usize) {
    match &exprs[index] {
        Expr::NoneStat | Expr::NoneExpr => (
            ValueType::Error(ErrorData {
                error: Error::NotParsed,
            }),
            global_index,
        ),
        Expr::Var { name_start, name } => todo!(),
        Expr::WordNum { str_len, end, .. } => (ValueType::Number(*str_len as i64), *end + 1),
        Expr::Operator {
            locs,
            func_type,
            indexes,
            end,
        } => match func_type {
            OperatorType::Add => todo!(),
            OperatorType::Sub => todo!(),
            OperatorType::Mult => todo!(),
            OperatorType::Div => todo!(),
            OperatorType::Mod => todo!(),
            OperatorType::Exp => todo!(),
            OperatorType::Log => todo!(),
        },
        Expr::LitNum {
            str_start,
            str_length,
            value,
        } => (ValueType::Number(*value), *str_start + *str_length),
        Expr::MultiLitNum {
            locs,
            num_indexes,
            end,
        } => todo!(),
        Expr::Skip {
            locs,
            index,
            start,
            end,
        } => {
            set_global_to_loc(&mut global_index, locs);
            run_expr(exprs, vars, *index, global_index)
        }

        // this should never run - but if it does than error
        _ => (
            ValueType::Error(ErrorData {
                error: Error::Malformed,
            }),
            global_index,
        ),
    }
}

// None = cant closes
// Some(..,0) = can close
// Some(..,1) = must close
type VaribleLengthFn = fn(&[f64]) -> Option<(Statement, bool)>;
fn run_variable_length_args(
    exprs: &ExprArena,
    vars: &Vars,
    indexes: &[usize],
    mut global_index: usize,
    close_fn: VaribleLengthFn,
) -> (Statement, usize) {
    let mut last_statement = None;
    let mut vec = Vec::with_capacity(indexes.len());
    for j in indexes {
        // end of arguments
        if *j == usize::MAX {
            break;
        }
        let data;
        (data, global_index) = run_expr(exprs, vars, *j, global_index);
        // get the value inside
        if let Some(val) = data.get_f64() {
            vec.push(val);
            // if error than return
        } else if let ValueType::Error(data) = data {
            return (Statement::Error(data, global_index), global_index);
            // else make error - type couldn't convert
        } else {
            return (
                Statement::Error(
                    ErrorData {
                        error: Error::TypeError,
                    },
                    global_index,
                ),
                global_index,
            );
        }
        let close = (close_fn)(&vec);
        if let Some((stat, must_close)) = close {
            if must_close {
                return (stat, global_index);
            } else {
                last_statement = Some(stat);
            }
        } else {
            last_statement = None;
        }
    }
    (
        last_statement.unwrap_or(Statement::Error(
            ErrorData {
                error: Error::Malformed,
            },
            global_index,
        )),
        global_index,
    )
}

fn set_global_to_loc(global_index: &mut usize, locs: &[usize]) {
    if let Some(last) = locs.last() {
        *global_index = last + 1;
    }
}

fn set_global_to_end(global_index: &mut usize, end: usize) {
    if end != usize::MAX {
        *global_index = end + 1;
    }
}
// fn run_3_or_4_args(exprs: &ExprArena, indexes: [usize; 4], mut global_index: usize,checker:fn()->Optional<(Statement,bool)>)-> (Result<Statement,ErrorData>, usize) {
//     let data = [0i64; 4];
//     let has_last = false;

// first 3
// for j in 0..3 {
//     if indexes[j] == usize::MAX {
//         return (Err(ErrorData {
//             error: Error::NotParsed,
//             index: global_index,
//         }),global_index);
//     }
//     if self.write_expr(exprs) {

//     }
// }
//     match (j==3,indexes[j] == usize::MAX){
//         (true, true) => {},
//         (true, false) => {has_last = false;},
//         (false, true) => { return (Statement::Error(ErrorData {
//             error: Error::NotParsed,
//             index: global_index,
//         }),global_index);
//     },
//         (false, false) => todo!(),
//     }
//     // if indexes[j] == usize::MAX {
//     //     if j!=3{
//     //         return Statement::Error(ErrorData {
//     //             error: Error::NotParsed,
//     //             index: global_index,
//     //         }),
//     //     }
//     // }

// (Ok((data,has_last)),global_index)
// todo!()
// }
