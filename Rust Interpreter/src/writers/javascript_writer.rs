use crate::{
    commands::*,
    parser::{multi_lit_num::VarOrInt, string_lit::VarOrStr},
};

#[allow(dead_code)]
pub fn write(exprs: &ExprArena, line_starts: &Vec<usize>) -> String {
    let mut str = "".to_string();
    let mut indent: usize = 0;
    for statement in line_starts {
        str += &write_expr(exprs, *statement, &mut indent);
        str += "\n";
    }
    str
}

fn get_indent(indent: &usize) -> String {
    let mut str = "".to_string();
    for _ in 0..*indent {
        str += "  "
    }
    str
}

fn write_expr(exprs: &ExprArena, index: usize, indent: &mut usize) -> String {
    match &exprs[index] {
        Expr::NoneStat => "(todo stat)".to_string(),
        Expr::NoneExpr => "(todo expr)".to_string(),
        Expr::Title { data } => {
            let author_str = data
                .authors
                .iter()
                .map(|name| String::from_utf8_lossy(&name.0))
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "`{}`;\n\"By {}\";\n",
                String::from_utf8_lossy(&data.title),
                author_str
            )
        }
        Expr::Assign {
            var,
            value_index,
            first,
            ..
        } => format!(
            "{}{}{}_var = {};",
            get_indent(indent),
            if *first { "let " } else { "" },
            String::from_utf8_lossy(&var.name),
            write_expr(exprs, *value_index, indent)
        ),
        Expr::Line { indexes, .. } => {
            format!(
                "{}draw_line({});",
                get_indent(indent),
                write_exprs(exprs, indexes, ", ", indent)
            )
        }
        Expr::Bezier { indexes, .. } => {
            format!(
                "{}draw_bezier({});",
                get_indent(indent),
                write_exprs(exprs, indexes, ", ", indent)
            )
        }
        Expr::Arc { indexes, .. } => {
            format!(
                "{}draw_ellipse({});",
                get_indent(indent),
                write_exprs(exprs, indexes, ", ", indent)
            )
        }
        Expr::Rect { indexes, .. } => {
            format!(
                "{}draw_rect({});",
                get_indent(indent),
                write_exprs(exprs, indexes, ", ", indent)
            )
        }
        Expr::Star { indexes, .. } => {
            format!(
                "{}draw_star({});",
                get_indent(indent),
                write_exprs(exprs, indexes, ", ", indent)
            )
        }
        Expr::Polygon { indexes, .. } => {
            format!(
                "{}draw_poly({});",
                get_indent(indent),
                write_exprs(exprs, indexes, ", ", indent)
            )
        }
        Expr::Triangle { indexes, .. } => {
            format!(
                "{}draw_tri({});",
                get_indent(indent),
                write_exprs(exprs, indexes, ", ", indent)
            )
        }
        Expr::Heart { indexes, .. } => {
            format!(
                "{}draw_heart({});",
                get_indent(indent),
                write_exprs(exprs, indexes, ", ", indent)
            )
        }
        Expr::RoundRec { indexes, .. } => {
            format!(
                "{}draw_round_rec({});",
                get_indent(indent),
                write_exprs(exprs, indexes, ", ", indent)
            )
        }
        Expr::Kirby { indexes, .. } => {
            format!(
                "{}draw_kirby({});",
                get_indent(indent),
                write_exprs(exprs, indexes, ", ", indent)
            )
        }
        Expr::Var { var } => format!("{}_var", String::from_utf8_lossy(&var.name).to_string()),
        Expr::WordNum { str_len, .. } => str_len.to_string(),
        Expr::Operator {
            func_type, indexes, ..
        } => {
            let ret = match func_type {
                OperatorType::Log => {
                    format!("log_base({})", write_exprs(exprs, indexes, ", ", indent))
                }
                OperatorType::Exp => {
                    if indexes.len() == 1 {
                        format!("(Math.E ** {})", write_expr(exprs, indexes[0], indent))
                    } else {
                        format!("({})", write_exprs(exprs, indexes, " ** ", indent))
                    }
                }
                OperatorType::Equals => {
                    let first_exp = write_expr(exprs, indexes[0], indent);
                    let mut r;
                    if indexes.len() > 2 {
                        r = format!(
                            "({} == {}",
                            first_exp,
                            write_expr(exprs, indexes[1], indent)
                        );
                        for index in &indexes[2..] {
                            if *index != usize::MAX {
                                r += " && ";
                                r += format!(
                                    "{} == {}",
                                    first_exp,
                                    write_expr(exprs, *index, indent)
                                )
                                .as_str();
                            }
                        }
                        r += ")";
                    } else {
                        r = format!("{} == {}", first_exp, write_expr(exprs, indexes[1], indent));
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
                1 => format!("({}{})", name, write_expr(exprs, indexes[0], indent)),
                2 => format!(
                    "({} {} {})",
                    write_expr(exprs, indexes[0], indent),
                    name,
                    write_expr(exprs, indexes[1], indent)
                ),
                _ => {
                    let mut ret = String::new();
                    ret += "(";
                    ret += write_expr(exprs, indexes[0], indent).as_str();
                    for i in &indexes[1..] {
                        ret += " ";
                        ret += name;
                        ret += " ";
                        ret += write_expr(exprs, *i, indent).as_str();
                    }
                    ret += ")";
                    ret
                }
            }
            // format!("{} {}", name, write_exprs(exprs, indexes))
        }
        Expr::LitNum { value, .. } => value.to_string(),
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
                format!(
                    "{}print_console(\"{}\");",
                    get_indent(indent),
                    String::from_utf8_lossy(word)
                )
            } else {
                format!(
                    "{}print_console({});",
                    get_indent(indent),
                    write_exprs(exprs, indexes, ", ", indent)
                )
            }
        }
        Expr::If { indexes, .. } => {
            let ind = get_indent(indent);
            *indent += 1;
            let str = format!(
                "{}if ({}) {{\n{}\n{}}}",
                ind,
                write_expr(exprs, indexes[0], indent),
                write_exprs(exprs, &indexes[1..], "\n", indent),
                ind
            );
            *indent -= 1;
            return str;
        }
        Expr::While { indexes, .. } => {
            let ind = get_indent(indent);
            *indent += 1;
            let str = format!(
                "{}while ({}) {{\n{}\n{}}}",
                ind,
                write_expr(exprs, indexes[0], indent),
                write_exprs(exprs, &indexes[1..], "\n", indent),
                ind
            );
            *indent -= 1;
            return str;
        }
        Expr::Else { indexes, .. } => {
            let ind = get_indent(indent);
            *indent += 1;
            let str = format!(
                "{}else {{\n{}\n{}}}",
                ind,
                write_exprs(exprs, indexes, "\n", indent),
                ind
            );
            *indent -= 1;
            return str;
        }
        Expr::LitCol { value, .. } => {
            format!("\"{}\"", String::from_utf8_lossy(&value))
        }
        Expr::Stroke { indexes, .. } => {
            if indexes[1] == usize::MAX {
                format!(
                    "{}set_stroke({});",
                    get_indent(indent),
                    write_expr(exprs, indexes[0], indent)
                )
            } else {
                format!(
                    "{}set_stroke({});",
                    get_indent(indent),
                    write_exprs(exprs, indexes, ", ", indent)
                )
            }
        }
        Expr::Fill { indexes, .. } => {
            if indexes[1] == usize::MAX {
                format!(
                    "{}set_fill({});",
                    get_indent(indent),
                    write_expr(exprs, indexes[0], indent)
                )
            } else {
                format!(
                    "{}set_fill({});",
                    get_indent(indent),
                    write_exprs(exprs, indexes, ", ", indent)
                )
            }
        }
        Expr::Color { indexes, .. } => {
            format!("get_color({})", write_exprs(exprs, indexes, ", ", indent))
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
            format!(
                "{}move_to({});",
                get_indent(indent),
                write_exprs(exprs, indexes, ", ", indent)
            )
        }
        Expr::LineWidth { child_index, .. } => {
            format!(
                "{}set_line_width({});",
                get_indent(indent),
                write_expr(exprs, *child_index, indent)
            )
        }
        Expr::Rotate { index, .. } => {
            format!(
                "{}rotate_delta({});",
                get_indent(indent),
                write_expr(exprs, *index, indent)
            )
        }
        Expr::Append { indexes, .. } => {
            if indexes[2] == usize::MAX {
                format!(
                    "{}{}.push({});",
                    get_indent(indent),
                    write_expr(exprs, indexes[0], indent),
                    write_expr(exprs, indexes[1], indent)
                )
            } else {
                format!(
                    "{}{}.splice({}, 0, {});",
                    get_indent(indent),
                    write_expr(exprs, indexes[0], indent),
                    write_expr(exprs, indexes[2], indent),
                    write_expr(exprs, indexes[1], indent)
                )
            }
        }
        Expr::Delete { indexes, .. } => {
            if indexes[1] == usize::MAX {
                format!(
                    "{}{}.splice(0,1);",
                    get_indent(indent),
                    write_expr(exprs, indexes[0], indent)
                )
            } else {
                format!(
                    "{}{}.splice({},1);",
                    get_indent(indent),
                    write_expr(exprs, indexes[0], indent),
                    write_expr(exprs, indexes[1], indent)
                )
            }
        }
        Expr::Replace { indexes, .. } => {
            format!(
                "{}{}[{}]={};",
                get_indent(indent),
                write_expr(exprs, indexes[0], indent),
                write_expr(exprs, indexes[1], indent),
                write_expr(exprs, indexes[2], indent)
            )
        }
        Expr::Find { indexes, .. } => {
            format!(
                "{}{}.indexOf({})",
                get_indent(indent),
                write_expr(exprs, indexes[0], indent),
                write_expr(exprs, indexes[1], indent)
            )
        }
        Expr::Index { indexes, .. } => {
            format!(
                "{}[{}]",
                write_expr(exprs, indexes[0], indent),
                write_expr(exprs, indexes[1], indent)
            )
        }
        Expr::List { indexes, .. } => {
            format!("[{}]", write_exprs(exprs, indexes, ", ", indent))
        }
        Expr::ForEach { indexes, var, .. } => {
            let ind = get_indent(indent);
            *indent += 1;
            let str = format!(
                "{}for(let {}_var of {}) {{\n{}\n{}}}",
                ind,
                String::from_utf8_lossy(&var.name),
                write_expr(exprs, indexes[0], indent),
                write_exprs(exprs, &indexes[1..], "\n", indent),
                ind
            );
            *indent -= 1;
            return str;
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

            *indent += 1;
            let str = format!(
                "function {}_var({}) {{\n{}\n}}",
                String::from_utf8_lossy(&func.name),
                args_str,
                write_exprs(exprs, indexes, "\n", indent)
            );
            *indent -= 1;
            return str;
        }
        Expr::FunctionCall { func, indexes, .. } => {
            //Trying without a semicolon since JS lets you forget them sometimes and function calls can be either expressions or statements
            format!(
                "{}_var({})",
                String::from_utf8_lossy(&func.name),
                write_exprs(exprs, indexes, ", ", indent)
            )
        }
        Expr::Return { index, .. } => {
            if let Some(ind) = index {
                format!(
                    "{}return {};",
                    get_indent(indent),
                    write_expr(exprs, *ind, indent)
                )
            } else {
                format!("{}return;", get_indent(indent))
            }
        }
        Expr::Length { index, .. } => {
            format!("{}.length", write_expr(exprs, *index, indent))
        }
        Expr::Not { .. } => {
            format!("")
        }
        Expr::Frame { .. } => {
            format!("_frame")
        }
        Expr::Comment { comment, .. } => {
            format!(
                "/* {} */",
                str::replace(&String::from_utf8_lossy(comment), "*/", "* /")
            )
        }
        Expr::Trig {
            func_type, index, ..
        } => {
            let name = match func_type {
                TrigType::Sin => "sin",
                TrigType::Cos => "cos",
                TrigType::Tan => "tan",
            };
            format!(
                "Math.{name}({}*Math.PI/180)",
                write_expr(exprs, *index, indent)
            )
        }
        Expr::Rand { indexes, .. } => {
            format!("get_random({})", write_exprs(exprs, indexes, ", ", indent))
        }
        Expr::Floor { index, .. } => {
            format!("Math.floor({})", write_expr(exprs, *index, indent))
        }
    }
}

fn write_exprs(
    exprs: &ExprArena,
    indexes: &[usize],
    delimeter: &str,
    indent: &mut usize,
) -> String {
    // if indexes.len() == 0 {
    //     return "".to_string();
    // }
    // let mut ret = String::new();
    // ret += write_expr(exprs, indexes[0], indent).as_str();
    // for index in &indexes[1..] {
    //     if *index != usize::MAX {
    //         ret += delimeter;
    //         ret += write_expr(exprs, *index, indent).as_str();
    //     }
    // }
    // ret
    indexes
        .into_iter()
        .filter_map(|&index| {
            (index != usize::MAX)
                .then(|| write_expr(exprs, index, indent))
                .filter(|e| !e.is_empty())
        })
        .collect::<Vec<_>>()
        .join(delimeter)
}
