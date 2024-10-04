use std::usize;

use crate::{commands::*, parser::{multi_lit_num::VarOrInt, End}};

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
            var,
            value_index,
            end,
            first,
        } => format!(
            "(assign{} {}{} {})",
            join_locs(locs, Some(*end)),
            if *first { "" } else { "mut " },
            write_var(var),
            write_expr(exprs, *value_index, 0)
        ),
        Expr::Bezier { locs, indexes, end } => {
            format!(
                "(bezier{} {})",
                join_locs(locs, Some(*end)),
                write_exprs(exprs, indexes),
            )
        }
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
        Expr::Var { var } => format!("(var {})", write_var(var)),
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
            single_value,
            values,
            end,
            ..
        } => {
            if let Some(intval) = single_value {
                format!("(litnum{} {})", join_locs(locs, Some(*end)), intval)
            } else {
                let mut output_vals = "".to_string();
                let mut is_first = true;
                for val in values {
                    if !is_first{
                        output_vals += " ";
                    }else{
                        is_first=false;
                    }
                    if let VarOrInt::Var(var) = val {
                        output_vals += &format!("{}", String::from_utf8_lossy(&var.name));
                    }else if let VarOrInt::Int(intval) = val{
                        output_vals += &format!("{}",intval);
                    }
                }
                format!(
                    "(multilitnum{} {})",
                    join_locs(locs, Some(*end)),
                    output_vals
                )
            }
        }
        Expr::Print { locs, data, end } => {
            format!(
                "(print{}{})",
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
                "(while{} {} then:\n{}\n)",
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
        Expr::LitCol {
            str_start,
            str_length,
            value,
        } => {
            format!(
                "(litcol {}@{}$${})",
                String::from_utf8_lossy(value),
                str_start,
                str_length
            )
        }
        Expr::Color { locs, indexes, end } => {
            format!(
                "(color{} {})",
                join_locs(locs, Some(*end)),
                write_exprs(exprs, indexes),
            )
        }
        Expr::Stroke { locs, indexes, end } => {
            format!(
                "(stroke{} {})",
                join_locs(locs, Some(*end)),
                write_exprs(exprs, indexes),
            )
        }
        Expr::Fill { locs, indexes, end } => {
            format!(
                "(fill{} {})",
                join_locs(locs, Some(*end)),
                write_exprs(exprs, indexes),
            )
        }
        Expr::LitString { str_start, str } => {
            format!(
                "(string${} \"{}\")",
                *str_start,
                String::from_utf8_lossy(str)
            )
        }
        Expr::MoveTo { locs, indexes, end } => {
            format!(
                "(moveto{} {})",
                join_locs(locs, Some(*end)),
                write_exprs(exprs, indexes),
            )
        }
        Expr::LineWidth {
            locs,
            child_index,
            end,
        } => {
            format!(
                "(linewidth{} {})",
                join_locs(locs, Some(*end)),
                write_expr(exprs, *child_index, 0)
            )
        }
        Expr::Rotate { locs, index, end } => {
            format!(
                "(rotate{} {})",
                join_locs(locs, Some(*end)),
                write_expr(exprs, *index, 0)
            )
        }
        Expr::Append { locs, indexes, end } => {
            format!(
                "(append{} {})",
                join_locs(locs, Some(*end)),
                write_exprs(exprs, indexes)
            )
        }
        Expr::Delete { locs, indexes, end } => {
            format!(
                "(delete{} {})",
                join_locs(locs, Some(*end)),
                write_exprs(exprs, indexes)
            )
        }
        Expr::Replace { locs, indexes, end } => {
            format!(
                "(replace{} {})",
                join_locs(locs, Some(*end)),
                write_exprs(exprs, indexes)
            )
        }
        Expr::Find { locs, indexes, end } => {
            format!(
                "(find{} {})",
                join_locs(locs, Some(*end)),
                write_exprs(exprs, indexes)
            )
        }
        Expr::Index { locs, indexes, end } => {
            format!(
                "(index{} {})",
                join_locs(locs, Some(*end)),
                write_exprs(exprs, indexes)
            )
        }
        Expr::List { locs, indexes, end } => {
            format!(
                "(list{} {})",
                join_locs(locs, Some(*end)),
                write_exprs(exprs, indexes)
            )
        }
        Expr::ForEach {
            locs,
            indexes,
            end,
            name,
            ..
        } => {
            let split = indexes.split_at_checked(1).unwrap_or_default();
            format!(
                "(foreach{} {} {} then:\n{}\n)",
                join_locs(locs, Some(*end)),
                String::from_utf8_lossy(&name),
                write_expr(exprs, *split.0.first().unwrap_or(&usize::MAX), 0),
                write_stats(exprs, split.1, indent + 1),
            )
        }
        Expr::Function {
            locs,
            name,
            arg_names,
            indexes,
            end,
            ..
        } => {
            let mut output_vals = "".to_string();
            let mut is_first = true;
            for val in arg_names {
                if is_first {
                    output_vals += &format!("{}", String::from_utf8_lossy(val));
                    is_first = false;
                } else {
                    output_vals += &format!(" {}", String::from_utf8_lossy(val));
                }
            }
            format!(
                "(function{} {} (args {}) {})",
                join_locs(locs, Some(*end)),
                String::from_utf8_lossy(name),
                output_vals,
                write_exprs(exprs, indexes)
            )
        }
        Expr::FunctionCall {
            locs,
            name,
            indexes,
            end,
            ..
        } => {
            format!(
                "({}{} {})",
                String::from_utf8_lossy(name),
                join_locs(locs, Some(*end)),
                write_exprs(exprs, indexes)
            )
        }
        Expr::Return { locs, index, end } => {
            if let Some(ind) = index {
                format!(
                    "(return{} {})",
                    join_locs(locs, Some(*end)),
                    write_expr(exprs, *ind, 0)
                )
            } else {
                format!("(return{})", join_locs(locs, Some(*end)))
            }
        }
        Expr::Length { locs, index, end } => {
            format!(
                "(length{} {})",
                join_locs(locs, Some(*end)),
                write_expr(exprs, *index, 0)
            )
        },
        Expr::Not { locs, word, str_start, str_len, end } => {
            format!(
                "(not{} @{}$${} {})",
                join_locs(locs, Some(*end)),
                *str_start,
                *str_len,
                String::from_utf8_lossy(word)
            )
        },
        Expr::Ignore { name_start, name } => format!(
            "(ignore \"{}\"@{})",
            String::from_utf8_lossy(&name).to_string(),
            name_start
        ),
    }
}

fn write_prints(exprs: &ExprArena, data: &Vec<Prints>) -> String {
    let mut ret = String::new();
    for print in data {
        ret += &match print {
            Prints::Var(index) | Prints::String(index) => {
                format!(" {}", write_expr(exprs, *index, 0))
            }
            Prints::Word(str, index) => {
                format!(" \"{}\"@{}", std::str::from_utf8(str).unwrap(), index)
            }
        }
    }
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

fn write_var(var: &Var) -> String {
    let mut skips_str = String::new();
    if !var.skip_indexes.is_empty() {
        skips_str += "|";
        skips_str += &var
            .skip_indexes
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join(",");
    }

    format!(
        "\"{}\"@{}{}",
        String::from_utf8_lossy(&var.name),
        var.start,
        skips_str
    )
}
