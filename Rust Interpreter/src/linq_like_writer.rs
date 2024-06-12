use std::usize;

use crate::commands::*;

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
        Expr::Eq {
            locs,
            name_start,
            name,
            value_index,
        } => format!(
            "(eq{} \"{}\"@{} {})",
            join_locs(locs),
            String::from_utf8_lossy(&name),
            name_start,
            write_expr(exprs, *value_index)
        ),
        Expr::Line {
            locs,
            indexes
        } => format!(
            "(line{} {} {} {} {})",
            join_locs(locs),
            write_expr(exprs, indexes[0]),
            write_expr(exprs, indexes[1]),
            write_expr(exprs, indexes[2]),
            write_expr(exprs, indexes[3])
        ),
        Expr::Circle {
            locs,
            indexes
        } => format!(
            "(circle{} {} {} {})",
            join_locs(locs),
            write_expr(exprs, indexes[0]),
            write_expr(exprs, indexes[1]),
            write_expr(exprs, indexes[2])
        ),
        Expr::Var { name_start, name } => format!(
            "(var \"{}\"@{})",
            String::from_utf8_lossy(&name).to_string(),
            name_start
        ),
        Expr::Num {
            locs,
            str_start,
            str,
        } => format!(
            "(num{} \"{}\"@{})",
            join_locs(locs),
            String::from_utf8_lossy(str),
            str_start
        ),
        Expr::BiFunction {
            locs,
            func_type,
            indexes,
            ..
        } => {
            let name = match func_type {
                BiFunctionType::Add => "add",
                BiFunctionType::Sub => "sub",
                BiFunctionType::Mult => "mult",
                BiFunctionType::Div => "div",
                BiFunctionType::Mod => "mod",
                BiFunctionType::Expr => "expr",
                BiFunctionType::Log => "log",
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
        Expr::Arc { locs, indexes } => todo!(),
        //expr => panic!("found {expr:?} which has no branch"),
    }
}

fn write_exprs(exprs: &ExprArena, indexes: &Vec<usize>) -> String {
    let mut ret = String::new();
    for index in indexes {
        ret += &(write_expr(exprs, *index) + " ");
    }
    ret.pop();
    ret
}
