use crate::commands::*;

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
        Expr::Assign {
            locs: _,
            name_start: _,
            name,
            value_index,
            end: _,
        } => format!(
            "{} = {};",
            String::from_utf8_lossy(&name),
            write_expr(exprs, *value_index)
        ),
        Expr::Line {
            locs: _,
            indexes,
            end: _,
        } => {
            format!("draw_line({});", write_exprs(exprs, indexes, ", "))
        }
        Expr::Arc {
            locs: _,
            indexes,
            end: _,
        } => {
            if indexes[3] == usize::MAX {
                format!(
                    "draw_ellipse({}, {});",
                    write_exprs(exprs, indexes, ", "),
                    write_expr(exprs, indexes[2])
                )
            } else {
                format!("draw_ellipse({});", write_exprs(exprs, indexes, ", "))
            }
        }
        Expr::Rect {
            locs: _,
            indexes,
            end: _,
        } => {
            if indexes[3] == usize::MAX {
                format!(
                    "draw_rect({}, {});",
                    write_exprs(exprs, indexes, ", "),
                    write_expr(exprs, indexes[2])
                )
            } else {
                format!("draw_rect({});", write_exprs(exprs, indexes, ", "))
            }
        }
        Expr::Var {
            name_start: _,
            name,
        } => String::from_utf8_lossy(&name).to_string(),
        Expr::WordNum {
            locs: _,
            str_start: _,
            str_len,
            end: _,
        } => str_len.to_string(),
        Expr::Operator {
            locs: _,
            func_type,
            indexes,
            end: _,
        } => {
            if matches!(func_type, OperatorType::Log) {
                return format!("log_base({})", write_exprs(exprs, indexes, ", "));
            }
            let name = match func_type {
                OperatorType::Add => "+",
                OperatorType::Sub => "-",
                OperatorType::Mult => "*",
                OperatorType::Div => "/",
                OperatorType::Mod => "%",
                OperatorType::Exp => "**",
                OperatorType::Log => "log",
                OperatorType::LessThan => "<",
                OperatorType::GreaterThan => ">",
                OperatorType::And => "&&",
                OperatorType::Or => "||",
                OperatorType::Equals => "==",
                OperatorType::Not => "!",
            };
            match indexes.len() {
                1 => format!("{}{}", name, write_expr(exprs, indexes[0])),
                2 => format!(
                    "({} {} {})",
                    write_expr(exprs, indexes[0]),
                    name,
                    write_expr(exprs, indexes[1])
                ),
                _ => {
                    let mut ret = String::new();
                    ret += "(";
                    ret += write_expr(exprs, indexes[0]).as_str();
                    for i in &indexes[1..] {
                        ret += " ";
                        ret += name;
                        ret += " ";
                        ret += write_expr(exprs, *i).as_str();
                    }
                    ret += ")";
                    ret
                }
            }
            // format!("{} {}", name, write_exprs(exprs, indexes))
        }
        Expr::LitNum {
            str_start: _,
            str_length: _,
            value,
        } => value.to_string(),
        Expr::MultiLitNum {
            locs: _,
            num_indexes,
            end: _,
        } => format!("{}", write_exprs(exprs, num_indexes, "")),
        Expr::Print {
            locs: _,
            data,
            end: _,
        } => {
            format!("print_console({});", write_prints(exprs, data))
        }
        Expr::Skip { .. } => "".to_string(),
    }
}

fn write_prints(exprs: &ExprArena, data: &Vec<Prints>) -> String {
    let mut ret = String::new();
    for print in data {
        ret += &match print {
            Prints::Var(index) => write_expr(exprs, *index),
            Prints::Word(str, _index) => {
                format!("\"{}\"", std::str::from_utf8(str).unwrap().to_string())
            }
        };
        ret += ", ";
    }
    ret.pop();
    ret.pop();
    ret
}

fn write_exprs(exprs: &ExprArena, indexes: &[usize], delimeter: &str) -> String {
    if indexes.len() == 1 {
        return write_expr(exprs, indexes[0]);
    }
    let mut ret = String::new();
    ret += write_expr(exprs, indexes[0]).as_str();
    for index in &indexes[1..] {
        if *index != usize::MAX {
            ret += delimeter;
            ret += write_expr(exprs, *index).as_str();
        }
    }
    ret
}
