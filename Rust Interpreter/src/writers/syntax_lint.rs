use crate::{commands::*, parser::ParserSourceIter};

use super::syntax_renderers::{Renderer, TermColor};

//colors
const BASE_COLOR: (TermColor, bool) = (TermColor::White, true);

const LOC_COLOR: [(TermColor, bool); 3] = [
    (TermColor::Yellow, true),
    (TermColor::Purple, true),
    (TermColor::Blue, true),
];

const STRING_COLOR: (TermColor, bool) = (TermColor::Black, true);

const VAR_COLOR: (TermColor, bool) = (TermColor::Cyan, true);
const NUM_COLOR: (TermColor, bool) = (TermColor::Green, true);

pub struct SyntaxLinter<T: Renderer> {
    renderer: T,
    index: usize,
}

#[allow(dead_code)]
impl<T: Renderer> SyntaxLinter<T> {
    pub fn new() -> Self {
        Self {
            renderer: Default::default(),
            index: 0,
        }
    }
    pub fn into_string(self) -> Vec<u8> {
        self.renderer.into_string()
    }
}

#[allow(dead_code)]
impl<T: Renderer> SyntaxLinter<T> {
    pub fn write(
        &mut self,
        exprs: &ExprArena,
        line_starts: &[usize],
        mut source: ParserSourceIter,
    ) {
        for statement in line_starts {
            self.write_expr(&mut source, exprs, *statement, 0);
        }
        self.write_rest(&mut source);
    }
}

fn get_n(source: &mut ParserSourceIter, num: usize) -> Result<Vec<u8>, Vec<u8>> {
    let ret: Vec<u8> = source.take(num).cloned().collect();
    if ret.len() == num {
        Ok(ret)
    } else {
        Err(ret)
    }
}

impl<T: Renderer> SyntaxLinter<T> {
    fn write_rest(&mut self, source: &mut ParserSourceIter) {
        let buf = get_n(source, usize::MAX).map_or_else(|e| e, |o| o);
        self.index += buf.len();
        self.renderer.add_with(&buf, BASE_COLOR);
    }
    fn write_up_to(&mut self, source: &mut ParserSourceIter, index: usize) {
        self.write_up_to_as(source, index, BASE_COLOR);
    }
    fn write_up_to_as(
        &mut self,
        source: &mut ParserSourceIter,
        index: usize,
        color: (TermColor, bool),
    ) {
        let num = index
            .checked_sub(self.index)
            .expect("index is before the writing index");
        let buf = get_n(source, num).expect("found end of buffer");
        self.renderer.add_with(&buf, color);
        self.index = index;
    }
    fn write_as(&mut self, source: &mut ParserSourceIter, num: usize, color: (TermColor, bool)) {
        let buf = get_n(source, num).expect("found end of buffer");
        self.renderer.add_with(&buf, color);
        self.index += num;
    }
    fn insert(&mut self, text: &[u8], color: (TermColor, bool)) {
        self.renderer.add_with(&text, color);
    }
}

impl<T: Renderer> SyntaxLinter<T> {
    fn write_prints(
        &mut self,
        source: &mut ParserSourceIter,
        exprs: &ExprArena,
        data: &Vec<Prints>,
    ) {
        for print in data {
            match print {
                // stack index is not used in vars
                Prints::Var(index) => self.write_expr(source, exprs, *index, 0),
                Prints::Word(str, index) => {
                    self.write_up_to(source, *index);
                    self.write_as(source, str.len(), STRING_COLOR);
                }
            }
        }
    }

    fn write_locs(&mut self, source: &mut ParserSourceIter, locs: &Vec<usize>, stack_index: usize) {
        let color = LOC_COLOR[stack_index % 3];
        for loc in locs {
            self.write_up_to(source, *loc);
            self.write_as(source, 1, color);
        }
    }

    fn write_end(&mut self, source: &mut ParserSourceIter, end: usize, stack_index: usize) {
        let color = LOC_COLOR[stack_index % 3];
        if end != usize::MAX {
            if self.index > end {
                self.insert(b"_", color);
            } else {
                self.write_up_to(source, end);
                self.write_as(source, 1, color);
            }
        }
    }

    fn write_exprs(
        &mut self,
        source: &mut ParserSourceIter,
        exprs: &ExprArena,
        indexes: &[usize],
        stack_index: usize,
    ) {
        for index in indexes {
            self.write_expr(source, exprs, *index, stack_index);
        }
    }

    fn write_expr(
        &mut self,
        source: &mut ParserSourceIter,
        exprs: &ExprArena,
        index: usize,
        stack_index: usize,
    ) {
        match &exprs[index] {
            Expr::Assign {
                locs,
                name_start,
                name,
                value_index,
                end,
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_up_to(source, *name_start);
                self.write_as(source, name.len(), VAR_COLOR);
                self.write_expr(source, exprs, *value_index, stack_index + 1);
                self.write_end(source, *end, stack_index);
            }
            Expr::Line { locs, indexes, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.write_end(source, *end, stack_index);
            }
            Expr::Arc { locs, indexes, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.write_end(source, *end, stack_index);
            }
            Expr::Rect { locs, indexes, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.write_end(source, *end, stack_index);
            }
            Expr::Var { name_start, name } => {
                self.write_up_to(source, *name_start);
                self.write_as(source, name.len(), VAR_COLOR);
            }
            Expr::WordNum {
                locs,
                str_start,
                str_len,
                end,
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_up_to(source, *str_start);
                self.write_as(source, *str_len, STRING_COLOR);
                self.write_end(source, *end, stack_index);
            }
            Expr::Operator {
                locs, indexes, end, ..
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.write_end(source, *end, stack_index);
            }
            Expr::LitNum {
                str_start,
                str_length,
                ..
            } => {
                self.write_up_to(source, *str_start);
                self.write_as(source, *str_length, NUM_COLOR);
            }
            Expr::MultiLitNum {
                locs,
                end,
                num_indexes,
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, num_indexes, stack_index + 1);
                self.write_end(source, *end, stack_index);
            }
            Expr::Print { locs, data, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_prints(source, exprs, data);
                self.write_end(source, *end, stack_index);
            }
            Expr::Skip {
                locs,
                index,
                start,
                end,
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_up_to(source, *start - 1);
                self.write_up_to_as(source, *end, STRING_COLOR);
                self.write_end(source, *end, stack_index);
                // same stack_index for same color
                self.write_expr(source, exprs, *index, stack_index)
            }
            Expr::If {
                locs, 
                body_end ,
                indexes,
                ..
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.write_end(source, *body_end, stack_index);
            }
            Expr::While {
                locs, 
                body_end ,
                indexes,
                ..
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.write_end(source, *body_end, stack_index);
            }
            
            Expr::NoneExpr | Expr::NoneStat => {}
        };
    }
}
