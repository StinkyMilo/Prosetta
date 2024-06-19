use std::io::BufRead;

use crate::{commands::*, parser::ParserSource};

use super::syntax_renderers::{Renderer, TermColor};

const BASE_COLOR: (TermColor, bool) = (TermColor::White, true);

const LOC_COLOR: [(TermColor, bool); 3] = [
    (TermColor::Yellow, true),
    (TermColor::Purple, true),
    (TermColor::Blue, true),
];

const STRING_COLOR: (TermColor, bool) = (TermColor::Yellow, false);

const VAR_COLOR: (TermColor, bool) = (TermColor::Green, false);
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
    pub fn write(&mut self, exprs: &ExprArena, line_starts: &Vec<usize>, source: &mut ParserSource) {
        for statement in line_starts {
            self.write_expr(source, exprs, *statement, 0);
        }
    }
}

impl<T: Renderer> SyntaxLinter<T> {
    fn write_up_to(&mut self, source: &mut ParserSource, index: usize) {
        self.write_up_to_as(source, index, BASE_COLOR);
    }
    fn write_up_to_as(&mut self, source: &mut ParserSource, index: usize, color: (TermColor, bool)) {
        let num = index
            .checked_sub(self.index)
            .expect("index is before the writing index");
        let mut buf = vec![0u8; num];
        source
            .read_exact(buf.as_mut())
            .expect("found end of buffer");
        self.renderer.add_with(&buf, color);
        self.index = index;
    }
    fn write_as(&mut self, source: &mut dyn BufRead, num: usize, color: (TermColor, bool)) {
        let mut buf = vec![0u8; num];
        source
            .read_exact(buf.as_mut())
            .expect("found end of buffer");
        self.renderer.add_with(&buf, color);
        self.index += num;
    }

    fn write_locs(&mut self, source: &mut dyn BufRead, locs: &Vec<usize>, stack_index: usize) {
        let color = LOC_COLOR[stack_index % 3];
        for loc in locs {
            self.write_up_to(source, *loc);
            self.write_up_to_as(source, *loc, color);
        }
    }

    fn write_exprs(
        &mut self,
        source: &mut dyn BufRead,
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
        source: &mut dyn BufRead,
        exprs: &ExprArena,
        index: usize,
        stack_index: usize,
    ) {
        match &exprs[index] {
            Expr::Set {
                locs,
                name_start,
                name,
                value_index,
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_up_to(source, *name_start);
                self.write_as(source, name.len(), VAR_COLOR);
                self.write_expr(source, exprs, *value_index, stack_index + 1)
            }
            Expr::Line { locs, indexes } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
            }
            Expr::Arc { locs, indexes } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
            }
            Expr::Rect { locs, indexes } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
            }
            Expr::Var { name_start, name } => {
                self.write_up_to(source, *name_start);
                self.write_as(source, name.len(), VAR_COLOR);
            }
            Expr::WordNum {
                locs,
                str_start,
                str,
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_up_to(source, *str_start);
                self.write_as(source, str.len(), STRING_COLOR);
            }
            Expr::Operator { locs, indexes, .. } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
            }
            Expr::LitNum {
                locs,
                str_start,
                str_length,
                ..
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_up_to(source, *str_start);
                self.write_as(source, *str_length, NUM_COLOR);
            }
            Expr::Print { locs, .. } => {
                self.write_locs(source, locs, stack_index);
            }
            Expr::NoneExpr | Expr::NoneStat => {}
        };
    }
}
