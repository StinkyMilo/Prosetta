use crate::{
    commands::*,
    parser::{End, ParserSourceIter, SubStrData},
};

use super::syntax_renderers::{Renderer, TermColor};

//colors
const BASE_COLOR: (TermColor, bool) = (TermColor::White, true);
// const IGNORE_COLOR: (TermColor, bool) = (TermColor::White, false);

const LOC_COLOR: [(TermColor, bool); 3] = [
    (TermColor::Yellow, true),
    (TermColor::Purple, true),
    (TermColor::Blue, true),
];

const STRING_COLOR: (TermColor, bool) = (TermColor::Black, true);

const VAR_COLOR: (TermColor, bool) = (TermColor::Cyan, true);
const NUM_COLOR: (TermColor, bool) = (TermColor::Green, true);

pub struct SyntaxLinter<T: Renderer> {
    /// the renderer
    renderer: T,
    /// the current writing index
    index: usize,
    /// the length of ending characters and vec of ending colors if they exist
    ends: Option<(u8, Vec<(TermColor, bool)>)>,
}

#[allow(dead_code)]
impl<T: Renderer> SyntaxLinter<T> {
    pub fn new() -> Self {
        Self {
            renderer: Default::default(),
            index: 0,
            ends: None,
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
            self.write_end(&mut source);
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
    fn get_n_or_error(source: &mut ParserSourceIter, num: usize) -> Vec<u8> {
        get_n(source, num).expect("writing should not find end of buffer")
    }
    fn write_up_to_as(
        &mut self,
        source: &mut ParserSourceIter,
        index: usize,
        color: (TermColor, bool),
    ) {
        let num = index.checked_sub(self.index).expect(&format!(
            "index {} should be after the writing index {}",
            index, self.index
        ));
        let buf = Self::get_n_or_error(source, num);
        self.renderer.add_with(&buf, color);
        self.index = index;
    }
    fn write_as(&mut self, source: &mut ParserSourceIter, num: usize, color: (TermColor, bool)) {
        let buf: Vec<u8> = Self::get_n_or_error(source, num);
        self.renderer.add_with(&buf, color);
        self.index += num;
    }
    // fn insert(&mut self, text: &[u8], color: (TermColor, bool)) {
    //     self.renderer.add_with(&text, color);
    // }
    fn write_end(&mut self, source: &mut ParserSourceIter) {
        if let Some(end) = self.ends.take() {
            // let num = index
            //     .checked_sub(self.index)
            //     .expect("index is before the end index");
            let buf = Self::get_n_or_error(source, end.0 as usize);
            self.renderer.add_with_mult(&buf, end.1);
            self.index += end.0 as usize;
        }
    }
}

impl<T: Renderer> SyntaxLinter<T> {
    fn write_var(
        &mut self,
        source: &mut ParserSourceIter,
        var: &SubStrData,
        color: (TermColor, bool),
    ) {
        self.write_up_to(source, var.start);
        for &index in &var.skip_indexes {
            self.write_up_to_as(source, var.start + index as usize, color);
            self.write_as(source, 1, BASE_COLOR);
        }
        let len = var.name.len() + var.skip_indexes.len();
        self.write_up_to_as(source, var.start + len, color);
    }

    fn write_locs(&mut self, source: &mut ParserSourceIter, locs: &Vec<usize>, stack_index: usize) {
        let color = LOC_COLOR[stack_index % 3];
        for loc in locs {
            self.write_up_to(source, *loc);
            self.write_as(source, 1, color);
        }
    }

    fn add_end(&mut self, source: &mut ParserSourceIter, end: End, stack_index: usize) {
        let color = LOC_COLOR[stack_index % 3];
        if end.index != usize::MAX {
            //let close_index = self.index - end.count as usize;

            // if close_index == self.index {

            // } else {
            //     //different close character
            // }

            // if passed close
            if self.ends.is_some() && end.index > self.index {
                self.write_end(source);
                self.write_up_to(source, end.index);
            //close is before index
            } else if end.index < self.index {
                unreachable!(
                    "close index {} should be after writing index {}",
                    end.index, self.index
                );
            }
            // setup close
            if let Some((_, vec)) = &mut self.ends {
                vec.push(color);
            } else {
                self.write_up_to(source, end.index);
                self.ends = Some((end.count, vec![color]));
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
        if index == usize::MAX {
            return;
        }
        self.write_end(source);
        match &exprs[index] {
            Expr::Assign {
                locs,
                var,
                value_index,
                end,
                ..
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_var(source, var, VAR_COLOR);
                self.write_expr(source, exprs, *value_index, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::Bezier { locs, indexes, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::Line { locs, indexes, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::Arc { locs, indexes, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::Rect { locs, indexes, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::Var { var } => self.write_var(source, var, VAR_COLOR),
            Expr::WordNum {
                locs,
                str_start,
                str_len,
                end,
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_up_to(source, *str_start);
                self.write_as(source, *str_len, STRING_COLOR);
                self.add_end(source, *end, stack_index);
            }
            Expr::Operator {
                locs, indexes, end, ..
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
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
                str_start,
                ..
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_up_to(source, *str_start);
                self.write_as(source, end.index - str_start, NUM_COLOR);
                self.add_end(source, *end, stack_index);
            }
            Expr::Print {
                locs, indexes, end, ..
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::If {
                locs,
                // body_end ,
                indexes,
                end,
                ..
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::While {
                locs,
                // body_end ,
                indexes,
                end,
                ..
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::LitCol {
                str_start,
                str_length,
                ..
            } => {
                self.write_up_to(source, *str_start);
                self.write_as(source, *str_length, STRING_COLOR);
            }
            Expr::Stroke { locs, indexes, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::Fill { locs, indexes, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::Color { locs, indexes, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }

            Expr::Else {
                locs, indexes, end, ..
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::LitString {
                str_start, str_end, ..
            } => {
                self.write_up_to(source, *str_start);
                //one for each quote
                //TODO: different color for variables
                self.write_as(source, str_end - str_start + 1, STRING_COLOR);
            }
            Expr::MoveTo { locs, indexes, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::LineWidth {
                locs,
                child_index,
                end,
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_expr(source, exprs, *child_index, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::Rotate { locs, index, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_expr(source, exprs, *index, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::Function {
                locs,
                indexes,
                end,
                func,
                ..
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_var(source, func, VAR_COLOR);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::Append { indexes, locs, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::FunctionCall {
                locs,
                indexes,
                end,
                func,
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_var(source, func, VAR_COLOR);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::Delete { indexes, locs, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::Return { locs, index, end } => {
                self.write_locs(source, locs, stack_index);
                if let Some(ind) = index {
                    self.write_expr(source, exprs, *ind, stack_index + 1);
                }
                self.add_end(source, *end, stack_index);
            }
            Expr::Replace { indexes, locs, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::Find { indexes, locs, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::Index { indexes, locs, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::List { locs, indexes, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::ForEach {
                locs,
                indexes,
                var,
                end,
                ..
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_var(source, var, VAR_COLOR);
                self.write_exprs(source, exprs, indexes, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::Length { locs, index, end } => {
                self.write_locs(source, locs, stack_index);
                self.write_expr(source, exprs, *index, stack_index + 1);
                self.add_end(source, *end, stack_index);
            }
            Expr::Not {
                locs,
                str_start,
                str_len,
                end,
                ..
            } => {
                self.write_locs(source, locs, stack_index);
                self.write_up_to(source, *str_start);
                self.write_as(source, *str_len, STRING_COLOR);
                self.add_end(source, *end, stack_index);
            }
            Expr::NoneExpr | Expr::NoneStat => {}
        };
    }
}

//The wizards were literally nine at most!
