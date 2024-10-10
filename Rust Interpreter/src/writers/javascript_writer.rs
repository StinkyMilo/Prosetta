use crate::{commands::*, parser::multi_lit_num::VarOrInt, parser::string_lit::VarOrStr};

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
            var,
            value_index,
            first,
            ..
        } => format!(
            "{}{}_var = {};",
            if *first { "let " } else { "" },
            String::from_utf8_lossy(&var.name),
            write_expr(exprs, *value_index)
        ),
        Expr::Line {
            locs: _,
            indexes,
            end: _,
        } => {
            format!("draw_line({});", write_exprs(exprs, indexes, ", "))
        }
        Expr::Bezier {
            locs: _,
            indexes,
            end: _,
        } => {
            format!("draw_bezier({});", write_exprs(exprs, indexes, ", "))
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
        Expr::Var { var } => format!("{}_var", String::from_utf8_lossy(&var.name).to_string()),
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
            let ret = match func_type {
                OperatorType::Log => format!("log_base({})", write_exprs(exprs, indexes, ", ")),
                OperatorType::Exp => {
                    if indexes.len() == 1 {
                        format!("(Math.E ** {})", write_expr(exprs, indexes[0]))
                    } else {
                        format!("({})", write_exprs(exprs, indexes, " ** "))
                    }
                }
                OperatorType::Equals => {
                    let first_exp = write_expr(exprs, indexes[0]);
                    let mut r;
                    if indexes.len() > 2 {
                        r = format!("({} == {}", first_exp, write_expr(exprs, indexes[1]));
                        for index in &indexes[2..] {
                            if *index != usize::MAX {
                                r += " && ";
                                r += format!("{} == {}", first_exp, write_expr(exprs, *index))
                                    .as_str();
                            }
                        }
                        r += ")";
                    } else {
                        r = format!("{} == {}", first_exp, write_expr(exprs, indexes[1]));
                    }
                    r
                }
                _ => "".to_string(),
            };
            if ret != "" {
                return ret;
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
                    if !is_first {
                        output_vals += ", ";
                    } else {
                        is_first = false;
                    }
                    if let VarOrInt::Var(var) = val {
                        output_vals += &format!("{}_var", String::from_utf8_lossy(&var.name));
                    } else if let VarOrInt::Int(intval) = val {
                        output_vals += &format!("{}", intval);
                    }
                }
                format!("get_concat_value({})", output_vals)
            }
        }
        Expr::Print {
            indexes,
            single_word,
            ..
        } => {
            if let Some(word) = single_word {
                format!("print_console(\"{}\");", String::from_utf8_lossy(word))
            } else {
                format!("print_console({});", write_exprs(exprs, indexes, ", "))
            }
        }
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
            let mut output: String = String::new();
            for val in str.iter() {
                if let VarOrStr::Var(var) = val {
                    let new_val = format!("${{{}_var}}", String::from_utf8_lossy(&var.name));
                    output += &new_val[..];
                } else if let VarOrStr::Str(str) = val {
                    let new_val = String::from_utf8_lossy(str);
                    output += &new_val[..];
                }
            }
            format!("`{}`", output)
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
        Expr::Append { indexes, .. } => {
            if indexes[2] == usize::MAX {
                format!(
                    "{}.push({});",
                    write_expr(exprs, indexes[0]),
                    write_expr(exprs, indexes[1])
                )
            } else {
                format!(
                    "{}.splice({}, 0, {});",
                    write_expr(exprs, indexes[0]),
                    write_expr(exprs, indexes[2]),
                    write_expr(exprs, indexes[1])
                )
            }
        }
        Expr::Delete { indexes, .. } => {
            if indexes[1] == usize::MAX {
                format!("{}.splice(0,1);", write_expr(exprs, indexes[0]))
            } else {
                format!(
                    "{}.splice({},1);",
                    write_expr(exprs, indexes[0]),
                    write_expr(exprs, indexes[1])
                )
            }
        }
        Expr::Replace { indexes, .. } => {
            format!(
                "{}[{}]={};",
                write_expr(exprs, indexes[0]),
                write_expr(exprs, indexes[1]),
                write_expr(exprs, indexes[2])
            )
        }
        Expr::Find { indexes, .. } => {
            format!(
                "{}.indexOf({})",
                write_expr(exprs, indexes[0]),
                write_expr(exprs, indexes[1])
            )
        }
        Expr::Index { indexes, .. } => {
            format!(
                "{}[{}]",
                write_expr(exprs, indexes[0]),
                write_expr(exprs, indexes[1])
            )
        }
        Expr::List { indexes, .. } => {
            format!("[{}]", write_exprs(exprs, indexes, ", "))
        }
        Expr::ForEach { indexes, var, .. } => {
            format!(
                "for(let {}_var of {}) {{\n{}\n}}",
                String::from_utf8_lossy(&var.name),
                write_expr(exprs, indexes[0]),
                write_exprs(exprs, &indexes[1..], "\n")
            )
        }
        Expr::Function {
            func,
            args,
            indexes,
            ..
        } => {
            let args_str = args
                .into_iter()
                .map(|data| String::from_utf8_lossy(&data.name) + "_var")
                .collect::<Vec<_>>()
                .join(", ");

            format!(
                "function {}_var({}){{\n{}\n}}",
                String::from_utf8_lossy(&func.name),
                args_str,
                write_exprs(exprs, indexes, "\n")
            )
        }
        Expr::FunctionCall { func, indexes, .. } => {
            //Trying without a semicolon since JS lets you forget them sometimes and function calls can be either expressions or statements
            format!(
                "{}_var({})",
                String::from_utf8_lossy(&func.name),
                write_exprs(exprs, indexes, ", ")
            )
        }
        Expr::Return { index, .. } => {
            if let Some(ind) = index {
                format!("return {};", write_expr(exprs, *ind))
            } else {
                format!("return;")
            }
        }
        Expr::Length { index, .. } => {
            format!("{}.length", write_expr(exprs, *index))
        }
        Expr::Not { .. } => {
            format!("")
        }
    }
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
