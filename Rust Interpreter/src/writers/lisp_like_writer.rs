use std::usize;

use crate::{
    commands::*,
    parser::{multi_lit_num::VarOrInt, string_lit::VarOrStr, End, SubStrData},
};

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
        Expr::Title { data } => {
            let author_str = data.authors.iter().fold(String::new(), |str, author| {
                format!(
                    "{str} \"{}\"@{}$${}",
                    String::from_utf8_lossy(&author.0),
                    author.1,
                    author.2
                )
            });

            let imports_str = data.imports.iter().fold(String::new(), |str, import| {
                format!("{str} {}@{}$${}", import.0.get_name(), import.1, import.2)
            });

            format!(
                "(title \"{}\" (authors{author_str})@{} (imports{imports_str}))",
                String::from_utf8_lossy(&data.title),
                data.by_start
            )
        }
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
            "(wordnum{} {}@{})",
            join_locs(locs, Some(*end)),
            *str_len,
            *str_start,
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
                    if !is_first {
                        output_vals += " ";
                    } else {
                        is_first = false;
                    }
                    if let VarOrInt::Var(var) = val {
                        output_vals += &format!("{}", String::from_utf8_lossy(&var.name));
                    } else if let VarOrInt::Int(intval) = val {
                        output_vals += &format!("{}", intval);
                    }
                }
                format!(
                    "(multilitnum{} {})",
                    join_locs(locs, Some(*end)),
                    output_vals
                )
            }
        }
        Expr::Print {
            locs,
            indexes,
            end,
            single_word,
            single_word_start,
            ..
        } => {
            if let Some(word) = single_word {
                format!(
                    "(print{} \"{}\"@{})",
                    join_locs(locs, Some(*end)),
                    String::from_utf8_lossy(word),
                    single_word_start
                )
            } else {
                format!(
                    "(print{} {})",
                    join_locs(locs, Some(*end)),
                    write_exprs(exprs, indexes)
                )
            }
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
        Expr::LitString { str, str_start, .. } => {
            let mut output: String = String::new();
            for val in str.iter() {
                if let VarOrStr::Var(var) = val {
                    let new_val = format!("{}", String::from_utf8_lossy(&var.name));
                    output += &new_val[..];
                } else if let VarOrStr::Str(str) = val {
                    let new_val = String::from_utf8_lossy(str);
                    output += &new_val[..];
                }
            }
            format!("\"{}\"@{}", output, str_start)
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
            var,
            ..
        } => {
            let split = indexes.split_at_checked(1).unwrap_or_default();
            format!(
                "(foreach{} {} {} then:\n{}\n)",
                join_locs(locs, Some(*end)),
                String::from_utf8_lossy(&var.name),
                write_expr(exprs, *split.0.first().unwrap_or(&usize::MAX), 0),
                write_stats(exprs, split.1, indent + 1),
            )
        }
        Expr::Function {
            locs,
            func,
            args,
            indexes,
            end,
            ..
        } => {
            let args_str = args
                .into_iter()
                .fold(String::new(), |str, arg| str + &" " + &write_var(arg));
            format!(
                "(function{} {} (args{}) {})",
                join_locs(locs, Some(*end)),
                write_var(func),
                args_str,
                write_exprs(exprs, indexes)
            )
        }
        Expr::FunctionCall {
            locs,
            func,
            indexes,
            end,
            ..
        } => {
            format!(
                "({}{} {})",
                write_var(func),
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
        }
        Expr::Not {
            locs,
            word,
            str_start,
            str_len,
            end,
        } => {
            format!(
                "(not{} @{}$${} \"{}\")",
                join_locs(locs, Some(*end)),
                *str_start,
                *str_len,
                String::from_utf8_lossy(word)
            )
        }
        Expr::Frame { locs } => {
            format!("(frame{})", join_locs(locs, None))
        },
        Expr::Comment { start, end, comment } => {
            format!("(comment@{}$${} \"{}\")",start,end,String::from_utf8_lossy(comment))
        }
    }
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

fn write_var(var: &SubStrData) -> String {
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
