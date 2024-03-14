use crate::commands::*;
#[allow(dead_code)]

pub fn write(exprs: &Vec<Expr>, line_starts: &Vec<usize>) -> String {
    let mut str = r#"void setup(){
    size(400, 400);
}

<T> T todo(){
    throw new java.lang.UnsupportedOperationException("Not parsed yet");
}

void pcircle(int x,int y,int r){
    circle(x,y,r*2);
}

void draw(){
    background(255);
"#
    .to_string();
    for statement in line_starts {
        str += "    ";
        str += &write_stat(exprs, *statement);
    }
    str + "}"
}

fn write_stat(exprs: &Vec<Expr>, start: usize) -> String {
    match &exprs[start] {
        Expr::NoneStat => ";\n".to_string(),
        Expr::Eq {
            name, value_index, ..
        } => format!(
            "var {}={};\n",
            String::from_utf8_lossy(&name),
            write_expr(exprs, *value_index)
        ),
        Expr::Line {
            x_index,
            y_index,
            x2_index,
            y2_index,
            ..
        } => format!(
            "line({},{},{},{});\n",
            write_expr(exprs, *x_index),
            write_expr(exprs, *y_index),
            write_expr(exprs, *x2_index),
            write_expr(exprs, *y2_index)
        ),
        Expr::Circle {
            x_index,
            y_index,
            r_index,
            ..
        } => format!(
            "pcircle({},{},{});\n",
            write_expr(exprs, *x_index),
            write_expr(exprs, *y_index),
            write_expr(exprs, *r_index)
        ),
        expr => panic!("found non starting expresion {expr:?} in starting position"),
    }
}

fn write_expr(exprs: &Vec<Expr>, index: usize) -> String {
    match &exprs[index] {
        Expr::NoneExpr => "todo()".to_string(),
        Expr::Var { name, .. } => String::from_utf8_lossy(&name).to_string(),
        Expr::Num { str, .. } => format!("\"{}\".length()", String::from_utf8_lossy(&str)),
        Expr::Mult {
            a_index, b_index, ..
        } => format!(
            "({})*({})",
            write_expr(exprs, *a_index),
            write_expr(exprs, *b_index)
        ),
        Expr::Add {
            a_index, b_index, ..
        } => format!(
            "{}+{}",
            write_expr(exprs, *a_index),
            write_expr(exprs, *b_index)
        ),
        expr => panic!("found starting expresion {expr:?} in non starting position"),
    }
}
