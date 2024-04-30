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
            join_locs(&locs),
            String::from_utf8_lossy(&name),
            name_start,
            write_expr(exprs, *value_index)
        ),
        Expr::Line {
            locs,
            x_index,
            y_index,
            x2_index,
            y2_index,
        } => format!(
            "(line{} {} {} {} {})",
            join_locs(&locs),
            write_expr(exprs, *x_index),
            write_expr(exprs, *y_index),
            write_expr(exprs, *x2_index),
            write_expr(exprs, *y2_index)
        ),
        Expr::Circle {
            locs,
            x_index,
            y_index,
            r_index,
        } => format!(
            "(circle{} {} {} {})",
            join_locs(&locs),
            write_expr(exprs, *x_index),
            write_expr(exprs, *y_index),
            write_expr(exprs, *r_index)
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
        Expr::Mult {
            locs,
            a_index,
            b_index,
        } => format!(
            "(mult{} {} {})",
            join_locs(locs),
            write_expr(exprs, *a_index),
            write_expr(exprs, *b_index)
        ),
        Expr::Add {
            locs,
            a_index,
            b_index,
        } => format!(
            "(add{} {} {})",
            join_locs(locs),
            write_expr(exprs, *a_index),
            write_expr(exprs, *b_index)
        ),
        Expr::Sub {
            locs,
            a_index,
            b_index,
        } => format!(
            "(sub{} {} {})",
            join_locs(locs),
            write_expr(exprs, *a_index),
            write_expr(exprs, *b_index)
        ),
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
        //expr => panic!("found {expr:?} which has no branch"),
    }
}
