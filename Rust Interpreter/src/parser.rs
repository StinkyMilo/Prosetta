#![allow(dead_code)]
#[path = "parser_source.rs"]
pub(crate) mod parser_source;
pub(crate) use parser_source::*;
// other stucts
#[path = "parser_structs.rs"]
pub(crate) mod parser_structs;
pub(crate) use parser_structs::*;

mod alias;
pub(crate) mod alias_data;
mod not;
mod operator;
mod set;
mod var;
mod word_num;

mod circle;
mod line;
mod print;
mod rect;

mod multi_lit_num;
mod num_literal;

#[path = "testing/parsing_tests_simple.rs"]
mod parsing_tests_simple;

#[path = "testing/parsing_tests_milo.rs"]
mod parsing_tests_milo;

#[path = "testing/parsing_tests_other.rs"]
mod parsing_tests_other;

use std::{fmt::Debug, hint::black_box, mem};

use crate::{commands::*, linq_like_writer};

use alias_data::AliasData;

//type MatchResult<T> = Option<(usize, T)>;
#[derive(Debug)]
pub struct ParsedData<'a> {
    pub exprs: ExprArena,
    pub stat_starts: Vec<usize>,
    pub vars: VarSet,
    pub source: ParserSource<'a>,
}

#[derive(Debug)]
pub struct Parser<'a> {
    pub data: ParsedData<'a>,
    stack: Vec<State>,
    last_state: Option<State>,

    parsing_line: bool,

    pos: usize,
    last_result: LastMatchResult,
    aliases: AliasData,
}

type ParseFunc = fn() -> MatchResult;
impl<'a> Parser<'a> {
    pub fn new(source: ParserSource<'a>, flags: ParserFlags) -> Self {
        Parser {
            data: ParsedData {
                exprs: ExprArena { vec: Vec::new() },
                stat_starts: Vec::new(),
                vars: VarSet::new(),
                source,
            },
            stack: Vec::new(),
            last_state: None,
            pos: 0,
            parsing_line: false,
            last_result: LastMatchResult::None,
            aliases: AliasData::new(flags),
        }
    }
    // pub fn change_source(&mut self, source: &'a mut dyn ParserSource) {
    //     self.source = source;
    // }

    pub fn get_last_state<'b>(&'b self) -> Option<&'b State> {
        self.last_state.as_ref().or_else(|| self.stack.last())
    }

    pub fn get_state(&self) -> &'static str {
        self.get_last_state()
            .map_or(&"None", |state| state.2.get_name())
    }

    pub fn get_word<'b>(&'b self) -> &'b [u8] {
        // (if self.last_result == LastMatchResult::Failed {
        //     self.get_last_state()
        // } else {
        //     self.stack.last()
        // })
        self.get_last_state().map_or(b"", |state| {
            Self::get_slice(self.data.source.get_line(), state.1).0.str
        })
    }
    pub fn into_data(mut self) -> ParsedData<'a> {
        self.data.source.drop_input();
        self.data
    }
}

impl<'a> Parser<'a> {
    pub fn step(&mut self) -> ParserResult {
        let is_first = !self.parsing_line;
        if is_first {
            // push match stat on first step of line
            if !self.setup_first() {
                return ParserResult::NoInput;
            } else {
                return ParserResult::Start;
            }
        }
        let _debug = format!(
            "{:?}",
            Vec::from_iter(self.stack.iter().map(|x| (x.0, x.1)))
        );
        let _debug2 = format!(
            "{:?}",
            Vec::from_iter(self.stack.iter().map(|x| x.2.get_name()))
        );
        let _expr = format!("{:?}", self.data.exprs.vec);
        let _expr2 = linq_like_writer::write(&self.data.exprs, &self.data.stat_starts);
        let _expr_short = format!(
            "{:?}",
            self.data.exprs.vec.iter().map(|e| {
                let mut str = format!("{:?}", e);
                str.truncate(str.find(" ").unwrap_or(str.len()));
                str
            })
        );
        let _last = format!("{:?}", self.last_result);
        black_box(&_debug);
        black_box(&_debug2);
        black_box(&_expr);
        black_box(&_expr2);

        self.last_state = None;
        // get curr frame
        let stack_index = self.stack.len() - 1;
        let frame = &mut self.stack[stack_index];
        let mut expr = &mut Expr::NoneExpr;

        //let next_child = self.data.exprs.vec.len();

        if frame.0 < self.data.exprs.vec.len() {
            expr = &mut self.data.exprs.vec[frame.0];
        }

        // setup env
        let mut env = Enviroment {
            expr,
            vars: &self.data.vars,
            locs: None,
            global_index: self.pos,
            aliases: &self.aliases,
        };

        // setup slice

        let (word, rest) = Self::get_slice(self.data.source.get_line(), frame.1);

        let last_result = mem::replace(&mut self.last_result, LastMatchResult::None);

        // run step function
        let mut result = match last_result {
            LastMatchResult::None | LastMatchResult::Continue => {
                frame.2.step(&mut env, &word, &rest)
            }
            LastMatchResult::New(locs) => {
                env.locs = locs;
                frame.2.step(&mut env, &word, &rest)
            }
            LastMatchResult::Matched(child_index) => {
                frame
                    .2
                    .step_match(&mut env, Some(child_index), &word, &rest)
            }
            LastMatchResult::Failed => frame.2.step_match(&mut env, None, &word, &rest),
        };

        // run aftermath
        let new_locs = env.locs.take();

        // reached end of line - upgrade result to failed
        if word.len() == 0 && matches!(result, MatchResult::Continue) {
            result = MatchResult::Failed;
        }

        match result {
            // I matched - return to last expr on stack with success
            MatchResult::Matched(index) => self.matched_func(index),
            // continue parsing child
            MatchResult::ContinueWith(index, state) => {
                self.continue_with_func(index, state, new_locs)
            }
            // continue with me
            MatchResult::Continue => self.continue_func(rest.pos),
            // I failed, go back on stack with fail
            MatchResult::Failed => self.failed_func(),
        }
    }

    fn failed_func(&mut self) -> ParserResult {
        let state = self.stack.pop().unwrap();

        let state_pos = state.0;
        self.data.exprs.vec.truncate(state_pos);
        //let _test = format!("{:?}", state);

        self.last_state = Some(state);

        self.last_result = LastMatchResult::Failed;
        // failed final stat - couldn't parse anything on line
        if self.stack.is_empty() {
            self.parsing_line = false;
            self.data.stat_starts.pop();
            ParserResult::FailedLine
        } else {
            // setup result for next step
            ParserResult::Failed
        }
    }

    fn continue_func(&mut self, new_index: usize) -> ParserResult {
        let stack_index = self.stack.len() - 1;
        let frame = &mut self.stack[stack_index];
        // change match starting location to after word
        frame.1 = new_index;

        self.last_result = LastMatchResult::Continue;

        ParserResult::Continue
    }

    fn continue_with_func(
        &mut self,
        index: usize,
        state: Box<dyn ParseState>,
        locs: Option<Vec<usize>>,
    ) -> ParserResult {
        let mut expr_index = self.data.exprs.vec.len();

        // replace none exprs
        if self.data.exprs.vec.last().is_some_and(|e| e.is_none()) {
            self.data.exprs.vec.pop();
            expr_index -= 1;
        }
        self.data.exprs.vec.push(Expr::NoneExpr);
        self.stack.push((expr_index, index, state));

        self.last_result = LastMatchResult::New(locs);

        ParserResult::ContinueWith
    }

    fn matched_func(&mut self, index: usize) -> ParserResult {
        let state = self.stack.pop().unwrap();
        let expr_index = state.0;
        self.last_state = Some(state);

        // matched final stat
        if self.stack.is_empty() {
            let start_index = *self.data.stat_starts.last().unwrap();
            self.parsing_line = false;
            // add to varibles
            if let Expr::Assign { name, .. } = &self.data.exprs[start_index] {
                self.data.vars.insert(name.to_owned());
            }
            ParserResult::MatchedLine
        } else {
            // setup result for next step
            self.last_result = LastMatchResult::Matched(expr_index);
            self.stack.last_mut().unwrap().1 = index;
            ParserResult::Matched
        }
    }

    fn get_slice(line: &[u8], start: usize) -> (Slice, Slice) {
        //let line = line.as_bytes();
        let slice = Slice {
            str: &line[start..],
            pos: start,
        };
        get_next_slice(&slice, 0)
    }

    fn setup_first(&mut self) -> bool {
        let line = self.data.source.get_line();
        self.pos += line.len();
        let data = self.data.source.new_line();
        if let Some(data) = data {
            let found_data = trim_ascii_whitespace(data).len() > 0;
            if found_data {
                // push match stat on first step of line
                let index = self.data.exprs.vec.len();

                self.data.exprs.vec.push(Expr::NoneStat);

                self.stack
                    .push((index, 0, Box::new(alias::NoneState::new_stat())));
                self.data.stat_starts.push(index);

                self.parsing_line = true;
                self.last_result = LastMatchResult::None;
            }
            found_data
        } else {
            false
        }
    }
}

// https://stackoverflow.com/questions/31101915/how-to-implement-trim-for-vecu8
pub fn trim_ascii_whitespace(x: &[u8]) -> &[u8] {
    let from = match x.iter().position(|x| !x.is_ascii_whitespace()) {
        Some(i) => i,
        None => return &x[0..0],
    };
    let to = x.iter().rposition(|x| !x.is_ascii_whitespace()).unwrap();
    &x[from..=to]
}
