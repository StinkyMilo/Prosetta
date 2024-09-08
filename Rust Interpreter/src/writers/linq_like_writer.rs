use std::usize;

use crate::{commands::*, parser::End};

fn write_end(end: End) -> String {
    let mut ret = String::new();
    if end.index != usize::MAX {
        ret += "$";
        ret += &end.index.to_string();
        if end.count != 1 {
            ret += "$$";
            ret += &end.count.to_string();
        }
    }
    ret
}

fn join_locs(locs: &Vec<usize>, end: Option<End>) -> String {
    if locs.is_empty() {
        "".to_string()
    } else {
        let mut iter = locs.into_iter();
        let first = iter.next().unwrap();
        let mut ret = iter.fold("@".to_string() + &first.to_string(), |a, b| {
            a + &"," + &b.to_string()
        });
        if let Some(end) = end {
            ret += &write_end(end);
        }
        ret
    }
}

#[allow(dead_code)]
pub fn write(exprs: &ExprArena, line_starts: &Vec<usize>) -> String {
    let mut str = "".to_string();
    for statement in line_starts {
        str += &write_expr(exprs, *statement, 0);
        str += "\n";
    }
    str.pop();
    str
}
#[allow(dead_code)]
pub fn write_first(exprs: &ExprArena) -> String {
    write_expr(exprs, 0, 0)
}

#[allow(dead_code)]
pub fn write_one(exprs: &ExprArena, index: usize) -> String {
    write_expr(exprs, index, 0)
}

fn write_expr(exprs: &ExprArena, index: usize, indent: usize) -> String {
    match &exprs[index] {
        Expr::NoneStat => "(todo stat)".to_string(),
        Expr::NoneExpr => "(todo expr)".to_string(),
        Expr::Assign {
            locs,
            name_start,
            name,
            value_index,
            end,
        } => format!(
            "(assign{} \"{}\"@{} {})",
            join_locs(locs, Some(*end)),
            String::from_utf8_lossy(&name),
            name_start,
            write_expr(exprs, *value_index, 0)
        ),
        Expr::Line { locs, indexes, end } => {
            format!(
                "(line{} {})",
                join_locs(locs, Some(*end)),
                write_exprs(exprs, indexes),
            )
        }
        Expr::Arc { locs, indexes, end } => {
            format!(
                "(arc{} {})",
                join_locs(locs, Some(*end)),
                write_exprs(exprs, indexes),
            )
        }
        Expr::Rect { locs, indexes, end } => {
            format!(
                "(rect{} {})",
                join_locs(locs, Some(*end)),
                write_exprs(exprs, indexes),
            )
        }
        Expr::Var { name_start, name } => format!(
            "(var \"{}\"@{})",
            String::from_utf8_lossy(&name).to_string(),
            name_start
        ),
        Expr::WordNum {
            locs,
            str_start,
            str_len,
            end,
        } => format!(
            "(wordnum{} @{}$${})",
            join_locs(locs, Some(*end)),
            *str_start,
            *str_len,
        ),
        Expr::Operator {
            locs,
            func_type,
            indexes,
            end,
        } => {
            let name = match func_type {
                OperatorType::Add => "+",
                OperatorType::Sub => "-",
                OperatorType::Mult => "*",
                OperatorType::Div => "/",
                OperatorType::Mod => "%",
                OperatorType::Exp => "exp",
                OperatorType::Log => "log",
                OperatorType::LessThan => "<",
                OperatorType::GreaterThan => ">",
                OperatorType::And => "&",
                OperatorType::Or => "||",
                OperatorType::Equals => "==",
                OperatorType::Not => "!",
            };
            format!(
                "({}{} {})",
                name,
                join_locs(locs, Some(*end)),
                write_exprs(exprs, indexes)
            )
        }
        Expr::LitNum {
            str_start,
            str_length,
            value,
        } => format!("(litnum {}@{}$${})", value, str_start, str_length),
        Expr::MultiLitNum {
            locs,
            num_indexes,
            end,
        } => format!(
            "(mutlilitnum{} {})",
            join_locs(locs, Some(*end)),
            write_exprs(exprs, num_indexes)
        ),
        Expr::Print { locs, data, end } => {
            format!(
                "(print{} {})",
                join_locs(locs, Some(*end)),
                write_prints(exprs, data)
            )
        }
        Expr::Skip {
            locs,
            index,
            start,
            end,
        } => {
            format!(
                "(skip{} @{}${} {})",
                join_locs(locs, None),
                *start,
                write_end(*end),
                write_expr(exprs, *index, 0),
            )
        }
        Expr::If {
            locs, indexes, end, ..
        } => {
            let split = indexes.split_at_checked(1).unwrap_or_default();
            format!(
                "(if{} {} then:\n{}\n)",
                join_locs(locs, Some(*end)),
                write_expr(exprs, *split.0.first().unwrap_or(&usize::MAX), 0),
                write_stats(exprs, split.1, indent + 1)
            )
        }
        Expr::While {
            locs, indexes, end, ..
        } => {
            let split = indexes.split_at_checked(1).unwrap_or_default();
            format!(
                "(while{} {} then {})",
                join_locs(locs, Some(*end)),
                write_expr(exprs, *split.0.first().unwrap_or(&usize::MAX), 0),
                write_stats(exprs, split.1, indent + 1),
            )
        }
        Expr::Else { locs, indexes, end } => {
            format!(
                "(else{}\n{}\n)",
                join_locs(locs, Some(*end)),
                write_stats(exprs, indexes, indent + 1)
            )
        }
    }
}

fn write_prints(exprs: &ExprArena, data: &Vec<Prints>) -> String {
    let mut ret = String::new();
    for print in data {
        ret += &match print {
            Prints::Var(index) => write_expr(exprs, *index, 0) + " ",
            Prints::Word(str, index) => {
                format!("\"{}\"@{} ", std::str::from_utf8(str).unwrap(), index)
            }
        }
    }
    ret.pop();
    ret
}

fn write_exprs(exprs: &ExprArena, indexes: &[usize]) -> String {
    write_mult_exprs(exprs, indexes, b' ', 0)
}
fn write_stats(exprs: &ExprArena, indexes: &[usize], indent: usize) -> String {
    write_mult_exprs(exprs, indexes, b'\n', indent)
}
fn write_mult_exprs(exprs: &ExprArena, indexes: &[usize], char: u8, indent: usize) -> String {
    let mut ret = String::new();
    for index in indexes {
        if *index != usize::MAX {
            for _ in 0..indent {
                ret += "  ";
            }
            ret += &write_expr(exprs, *index, indent);
            ret.push(char as char);
        }
    }
    ret.pop();
    ret
}
