use crate::commands::*;
use bitflags::bitflags;
#[allow(dead_code)]

//from
//https://docs.oracle.com/javase/tutorial/java/nutsandbolts/operators.html
// Operator Precedence
// additive -> 10 * 2
// multiplicative -> 11*2

const TODO_FUNC: &str = r#"
<T> T todo(){
    throw new java.lang.UnsupportedOperationException("Not parsed yet");
}"#;

const CIRCLE_FUNC: &str = r#"
void pcircle(int x,int y,int r){
    circle(x,y,r*2);
}"#;

const FUNCS: [&str; 2] = [TODO_FUNC, CIRCLE_FUNC];

bitflags! {
    /// Represents a set of flags.
    #[derive(Debug, Default)]
    struct FuncFlags: u32 {
        const uses_todo = 1;
        const uses_circle = 2;
    }
}

fn write_functions(flags: FuncFlags, str: &mut String) {
    for flag in flags {
        *str += FUNCS[flag.bits() as usize];
    }
}

pub fn write(exprs: &ExprArena, line_starts: &Vec<usize>) -> String {
    let mut flags = Default::default();
    let mut str = "void setup(){
        size(400, 400);
    }"
    .to_string();

    let mut code = "
void draw(){ 
    background(255);
"
    .to_string();
    for statement in line_starts {
        code += "    ";
        code += &write_stat(exprs, *statement, &mut flags);
    }

    write_functions(flags, &mut str);

    str + &code + "}"
}

fn write_stat(exprs: &ExprArena, start: usize, flags: &mut FuncFlags) -> String {
    match &exprs[start] {
        Expr::NoneStat => ";\n".to_string(),
        Expr::Set {
            name, value_index, ..
        } => format!(
            "var {}={};\n",
            String::from_utf8_lossy(&name),
            write_expr(exprs, *value_index, 0, flags)
        ),
        Expr::Line { indexes, .. } => format!(
            "line({},{},{},{});\n",
            write_expr(exprs, indexes[0], 0, flags),
            write_expr(exprs, indexes[1], 0, flags),
            write_expr(exprs, indexes[2], 0, flags),
            write_expr(exprs, indexes[3], 0, flags)
        ),
        Expr::Arc { indexes, .. } => {
            *flags |= FuncFlags::uses_circle;
            format!(
                "pcircle({},{},{});\n",
                write_expr(exprs, indexes[0], 0, flags),
                write_expr(exprs, indexes[1], 0, flags),
                write_expr(exprs, indexes[2], 0, flags)
            )
        }
        expr => panic!("found non starting expresion {expr:?} in starting position"),
    }
}

fn write_expr(
    exprs: &ExprArena,
    index: usize,
    last_precedence: u8,
    flags: &mut FuncFlags,
) -> String {
    match &exprs[index] {
        Expr::NoneExpr => {
            *flags |= FuncFlags::uses_todo;
            "todo()".to_string()
        }
        Expr::Var { name, .. } => String::from_utf8_lossy(&name).to_string(),
        Expr::WordNum { str, .. } => format!("\"{}\".length()", String::from_utf8_lossy(&str)),
        Expr::Operator {
            func_type, indexes, ..
        } => {
            let func_data = match func_type {
                OperatorType::Add => ("+", 10 * 2),
                OperatorType::Sub => ("-", 10 * 2),
                OperatorType::Mult => ("*", 11 * 2),
                OperatorType::Div => ("/",11*2),
                OperatorType::Mod => ("%",11*2),
            };
            write_var_len_operator(
                exprs,
                indexes,
                func_data.0,
                last_precedence,
                func_data.1,
                flags,
            )
        }

        Expr::LitNum { value, .. } => format!("{}", value),
        expr => panic!("found starting expresion {expr:?} in non starting position"),
    }
}

fn write_var_len_operator(
    exprs: &ExprArena,
    indexes: &Vec<usize>,
    operator: &str,
    last_precedence: u8,
    current_precedence: u8,
    flags: &mut FuncFlags,
) -> String {
    if indexes.len() < 2 {
        panic!("operator {operator} had less than two arguments");
    }
    let add_paren = last_precedence > current_precedence;
    let mut ret = String::new();
    if add_paren {
        ret += "(";
    }

    ret += &write_expr(exprs, indexes[0], current_precedence, flags);

    for index in indexes.iter().skip(1) {
        ret += operator;
        ret += &write_expr(exprs, *index, current_precedence + 1, flags);
    }

    if add_paren {
        ret += ")";
    }
    ret
}
