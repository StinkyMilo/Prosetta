use std::{fmt::format, usize};

use crate::commands::*;

use super::syntax_renderers::Renderer;

struct SyntaxLinter {
    renderer: Box<dyn Renderer>,
}

impl SyntaxLinter {
    pub fn new(renderer: Box<dyn Renderer>) -> Self {
        Self { renderer }
    }
    pub fn into_string(self) -> String {
        self.renderer.into_string()
    }
}

fn join_locs(locs: &Vec<usize>) -> String {
    if locs.is_empty() {
        "".to_string()
    } else {
        let mut iter = locs.into_iter();
        let first = iter.next().unwrap();
        iter.fold("@".to_string() + &first.to_string(), |a, b| {
            a + &"," + &b.to_string()
        })
    }
}

#[allow(dead_code)]
pub fn write(exprs: &ExprArena, line_starts: &Vec<usize>) -> String {
    let mut str = "".to_string();
    for statement in line_starts {
        str += &write_expr(exprs, *statement);
        str += "\n";
    }
    str
}
#[allow(dead_code)]
pub fn write_first(exprs: &ExprArena) -> String {
    write_expr(exprs, 0)
}

#[allow(dead_code)]
pub fn write_one(exprs: &ExprArena, index: usize) -> String {
    write_expr(exprs, index)
}

fn write_expr(exprs: &ExprArena, index: usize) -> String {
    match &exprs[index] {
        Expr::NoneStat => "(todo stat)".to_string(),
        Expr::NoneExpr => "(todo expr)".to_string(),
        Expr::Set {
            locs,
            name_start,
            name,
            value_index,
        } => format!(
            "(set{} \"{}\"@{} {})",
            join_locs(locs),
            String::from_utf8_lossy(&name),
            name_start,
            write_expr(exprs, *value_index)
        ),
        Expr::Line { locs, indexes } => {
            format!("(line{} {})", join_locs(locs), write_exprs(exprs, indexes),)
        }
        Expr::Arc { locs, indexes } => {
            format!("(arc{} {})", join_locs(locs), write_exprs(exprs, indexes),)
        }
        Expr::Rect { locs, indexes } => {
            format!("(rect{} {})", join_locs(locs), write_exprs(exprs, indexes),)
        }
        Expr::Var { name_start, name } => format!(
            "(var \"{}\"@{})",
            String::from_utf8_lossy(&name).to_string(),
            name_start
        ),
        Expr::WordNum {
            locs,
            str_start,
            str,
        } => format!(
            "(wordnum{} \"{}\"@{})",
            join_locs(locs),
            String::from_utf8_lossy(str),
            str_start
        ),
        Expr::Operator {
            locs,
            func_type,
            indexes,
            ..
        } => {
            let name = match func_type {
                OperatorType::Add => "add",
                OperatorType::Sub => "sub",
                OperatorType::Mult => "mult",
                OperatorType::Div => "div",
                OperatorType::Mod => "mod",
                OperatorType::Exp => "exp",
                OperatorType::Log => "log",
            };
            format!(
                "({}{} {})",
                name,
                join_locs(locs),
                write_exprs(exprs, indexes)
            )
        }
        Expr::LitNum {
            locs,
            str_start,
            str_length,
            value,
        } => format!(
            "(litnum{} {}@{}${})",
            join_locs(locs),
            value,
            str_start,
            str_length
        ),
        Expr::Print { locs, data } => {
            format!("(print{} {})", join_locs(locs), write_prints(exprs, data))
        }
    }
}

fn write_prints(exprs: &ExprArena, data: &Vec<Prints>) -> String {
    let mut ret = String::new();
    for print in data {
        ret += &match print {
            Prints::Var(index) => write_expr(exprs, *index) + " ",
            Prints::Word(str, index) => format!("\"{}\"@{} ", str, index),
        }
    }
    ret.pop();
    ret
}

fn write_exprs(exprs: &ExprArena, indexes: &[usize]) -> String {
    let mut ret = String::new();
    for index in indexes {
        if *index != usize::MAX {
            ret += &(write_expr(exprs, *index) + " ");
        }
    }
    ret.pop();
    ret
}
