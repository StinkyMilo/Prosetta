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
            name, value_index, ..
        } => format!(
            "{}mario = {};",
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
            format!("draw_ellipse({});", write_exprs(exprs, indexes, ", "))
        }
        Expr::Rect {
            locs: _,
            indexes,
            end: _,
        } => {
            format!("draw_rect({});", write_exprs(exprs, indexes, ", "))
        }
        Expr::Var {
            name_start: _,
            name,
        } => String::from_utf8_lossy(&name).to_string() + "mario",
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
            values,
            single_value,
            ..
        } => {
            if let Some(intval) = single_value {
                format!("{}", intval)
            } else {
                let mut output_vals = "".to_string();
                let mut is_first = true;
                for val in values {
                    if is_first {
                        output_vals += &format!("{}", val);
                        is_first = false;
                    } else {
                        output_vals += &format!(", {}", val);
                    }
                }
                format!("get_concat_value({})", output_vals)
            }
        }
        Expr::Print {
            locs: _,
            data,
            end: _,
        } => {
            format!("print_console({});", write_prints(exprs, data))
        }
        Expr::Skip { .. } => "".to_string(),
        Expr::If { indexes, .. } => {
            format!(
                "if ({}) {{\n{}\n}}",
                write_expr(exprs, indexes[0]),
                write_exprs(exprs, &indexes[1..], "\n")
            )
        }
        Expr::While { indexes, .. } => {
            format!(
                "while ({}) {{\n{}\n}}",
                write_expr(exprs, indexes[0]),
                write_exprs(exprs, &indexes[1..], "\n")
            )
        }
        Expr::Else { indexes, .. } => {
            format!("else {{\n{}\n}}", write_exprs(exprs, indexes, "\n"))
        }
        Expr::LitCol { value, .. } => {
            format!("\"{}\"", String::from_utf8_lossy(&value))
        }
        Expr::Stroke { indexes, .. } => {
            if indexes[1] == usize::MAX {
                format!("set_stroke({});", write_expr(exprs, indexes[0]))
            } else {
                format!("set_stroke({});", write_exprs(exprs, indexes, ", "))
            }
        }
        Expr::Fill { indexes, .. } => {
            if indexes[1] == usize::MAX {
                format!("set_fill({});", write_expr(exprs, indexes[0]))
            } else {
                format!("set_fill({});", write_exprs(exprs, indexes, ", "))
            }
        }
        Expr::Color { indexes, .. } => {
            format!("get_color({})", write_exprs(exprs, indexes, ", "))
        }
        Expr::LitString { str, .. } => {
            format!("\"{}\"", String::from_utf8_lossy(str))
        }
        Expr::MoveTo { indexes, .. } => {
            format!("move_to({});", write_exprs(exprs, indexes, ", "))
        }
        Expr::LineWidth { child_index, .. } => {
            format!("set_line_width({});", write_expr(exprs, *child_index))
        }
        Expr::Rotate { index, .. } => {
            format!("rotate_delta({});", write_expr(exprs, *index))
        }
    }
}

fn write_prints(exprs: &ExprArena, data: &Vec<Prints>) -> String {
    let mut ret = String::new();
    for print in data {
        ret += &match print {
            Prints::Var(index) | Prints::String(index) => write_expr(exprs, *index),
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
    if indexes.len() == 0 {
        return "".to_string();
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
