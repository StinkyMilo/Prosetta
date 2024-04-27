#![allow(dead_code)]

mod add_mult;
mod builtins;
mod builtins_data;
mod eq;
mod num;
mod var;

mod circle;
mod line;

#[path = "parsing_tests.rs"]
mod tests;

use std::{
    collections::HashSet,
    fmt::{self, Debug},
    hint::black_box,
    io::BufRead,
};

use crate::{commands::*, linq_like_writer};

//type MatchResult<T> = Option<(usize, T)>;
type VarSet = HashSet<Vec<u8>>;
//type ParseFn = fn(this: &Parser,&VarSet, &Slice<'_>, Vec<usize>) -> MatchResult;
type StepFunction =
    fn(env: &mut Enviroment, result: LastMatchResult, word: &Slice, rest: &Slice) -> MatchResult;

pub trait ParseSource: BufRead + Debug {}
impl<T: BufRead + Debug> ParseSource for T {}

#[derive(PartialEq, Debug)]
struct BuiltinMatchState {
    progress: Vec<u8>,
    locs: Vec<Option<Vec<usize>>>,
    offset: usize,
    matched: u16,
    is_expr: bool,
}

// #[derive(PartialEq, Debug)]
// enum StateContext {
//     None,
//     MultStep { step_num: u8 },
//     Matching(BuiltinMatchState),
// }
// impl Default for StateContext {
//     fn default() -> Self {
//         StateContext::None
//     }
// }
trait ParseState: Debug {
    /// call first time to setup the state
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult;

    /// call rest of times after match
    fn step_match(
        &mut self,
        env: &mut Enviroment,
        did_child_match: bool,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult;
    fn get_name(&self) -> &'static str;
    fn do_replace(&self) -> bool;
}

#[derive(Debug)]
enum MatchResult {
    Matched(usize),
    Continue(usize, Box<dyn ParseState>),
    ContinueFail,
    Failed,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum LastMatchResult {
    None,
    Matched,
    Failed,
    Continue,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ParserResult {
    NoInput,

    MatchedLine(&'static str),
    FailedLine(&'static str),

    Matched(&'static str),
    Continue(&'static str),
    ContinueFail(&'static str),
    Failed(&'static str),
}

struct Enviroment<'a> {
    vars: &'a VarSet,
    expr: &'a mut Expr,
    locs: Option<Vec<usize>>,
    child_index: usize,
    global_index: usize,
}

#[derive(PartialEq)]
struct Slice<'a> {
    str: &'a [u8],
    pos: usize,
}

impl<'a> fmt::Debug for Slice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Slice")
            .field("str", &String::from_utf8_lossy(&self.str))
            .field("pos", &self.pos)
            .finish()
    }
}

impl<'a> Slice<'a> {
    fn len(&self) -> usize {
        self.str.len()
    }
    fn end(&self) -> usize {
        self.pos + self.str.len()
    }
    fn offset(&self, offset: usize) -> Slice {
        Slice {
            str: &self.str[offset..],
            pos: self.pos + offset,
        }
    }
    fn extend(&self) -> Slice {
        Slice {
            str: &self.str[self.pos..],
            pos: self.pos,
        }
    }
}

fn get_next_word<'a>(slice: &'a Slice<'a>, mut start: usize) -> (Slice<'a>, Slice<'a>) {
    // find start of word
    start = start.min(slice.len());
    while start < slice.len() && !slice.str[start].is_ascii_alphanumeric() {
        start += 1;
    }

    // find end of word
    let mut end = start;
    while end < slice.len() && slice.str[end].is_ascii_alphanumeric() {
        end += 1;
    }

    (
        Slice {
            str: &slice.str[start..end],
            pos: slice.pos + start,
        },
        Slice {
            str: &slice.str[end..],
            pos: slice.pos + end,
        },
    )
}

// returns the rest after the end of the word
fn find_word_end<'a>(slice: &'a Slice<'a>, start: usize) -> Slice<'a> {
    // find end of word

    let mut end = start.min(slice.len());
    while end < slice.len() && slice.str[end].is_ascii_alphanumeric() {
        end += 1;
    }
    //let test = end < slice.len();
    //end = end.min(slice.len());
    Slice {
        str: &slice.str[end..],
        pos: slice.pos + end,
    }
}

// returns the rest after finding the end of an h word
fn find_h_close<'a>(slice: &'a Slice<'a>, start: usize) -> Option<Slice<'_>> {
    // find h
    let mut end = start;
    while end < slice.len() && slice.str[end] != b'h' && slice.str[end] != b'H' {
        end += 1;
    }
    let test = end < slice.len();
    // find end of h word
    test.then(|| find_word_end(slice, end))
}

#[derive(Debug)]
pub struct Parser<'a> {
    pub exprs: ExprArena,
    pub stat_starts: Vec<usize>,

    // (expr_index, string_index, state)
    stack: Vec<(usize, usize, Box<dyn ParseState>)>,

    pub vars: VarSet,

    curr_line: String,
    source: &'a mut dyn ParseSource,
    parsing_line: bool,

    pos: usize,
    last_result: LastMatchResult,
    last_locs: Option<Vec<usize>>,
}

type ParseFunc = fn() -> MatchResult;
impl<'a> Parser<'a> {
    pub fn new(source: &'a mut dyn ParseSource) -> Self {
        Parser {
            exprs: ExprArena { vec: Vec::new() },
            stack: Vec::new(),
            vars: VarSet::new(),
            stat_starts: Vec::new(),
            curr_line: String::new(),
            pos: 0,
            source,
            parsing_line: false,
            last_result: LastMatchResult::None,
            last_locs: None,
        }
    }
    pub fn change_source(&mut self, source: &'a mut dyn ParseSource) {
        self.source = source;
    }

    fn setup_first(&mut self) -> bool {
        self.pos += self.curr_line.len();
        self.curr_line = String::new();
        let has_read = self.source.read_line(&mut self.curr_line).is_ok();
        let found_data = has_read && self.curr_line.trim().len() > 0;
        if found_data {
            // push match stat on first step of line
            let index = self.exprs.vec.len();

            self.exprs.vec.push(Expr::NoneStat);

            self.stack
                .push((index, 0, Box::new(builtins::NoneState::new_stat())));
            self.stat_starts.push(index);

            self.parsing_line = true;
            self.last_locs = None;
            self.last_result = LastMatchResult::None;
        }
        found_data
    }

    pub fn step(&mut self) -> ParserResult {
        let is_first = !self.parsing_line;
        if is_first {
            // push match stat on first step of line
            if !self.setup_first() {
                return ParserResult::NoInput;
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
        let _expr = format!("{:?}", self.exprs.vec);
        let _expr2 = linq_like_writer::write(&self.exprs, &self.stat_starts);
        let _last = format!("{:?}", self.last_result);
        black_box(&_debug);
        black_box(&_debug2);
        black_box(&_expr);
        black_box(&_expr2);
        // get curr frame
        let stack_index = self.stack.len() - 1;
        let frame = &mut self.stack[stack_index];
        let mut expr = &mut Expr::NoneExpr;

        let next_child = self.exprs.vec.len();

        if frame.0 < self.exprs.vec.len() {
            expr = &mut self.exprs.vec[frame.0];
        }
        // setup env
        let mut env = Enviroment {
            expr,
            vars: &self.vars,
            locs: self.last_locs.take(),
            child_index: next_child,
            global_index: self.pos,
        };

        // setup slice
        let line = self.curr_line.as_bytes();
        let start = frame.1;
        let slice = Slice {
            str: &line[start..],
            pos: start,
        };

        let (word, rest) = get_next_word(&slice, 0);

        // run step function
        let mut result = match self.last_result {
            LastMatchResult::None | LastMatchResult::Continue => {
                frame.2.step(&mut env, &word, &rest)
            }
            LastMatchResult::Matched => frame.2.step_match(&mut env, true, &word, &rest),
            LastMatchResult::Failed => frame.2.step_match(&mut env, false, &word, &rest),
        };

        // run aftermath

        self.last_locs = env.locs.take();

        // reached end of line - upgrade result to failed
        if word.len() == 0 && matches!(result, MatchResult::ContinueFail) {
            result = MatchResult::Failed;
        }

        match result {
            // I matched - return to last expr on stack with success
            MatchResult::Matched(index) => {
                let state = self.stack.pop().unwrap();
                //let state = self.stack_state.pop().unwrap();
                //let expr = &self.exprs[expr_index];

                // matched final stat
                if self.stack.is_empty() {
                    let start_index = *self.stat_starts.last().unwrap();
                    self.parsing_line = false;
                    // add to varibles
                    if let Expr::Eq { name, .. } = &self.exprs[start_index] {
                        self.vars.insert(name.to_owned());
                    }
                    ParserResult::MatchedLine(state.2.get_name())
                } else {
                    // setup result for next step
                    self.last_result = LastMatchResult::Matched;
                    self.stack.last_mut().unwrap().1 = index;
                    ParserResult::Matched(state.2.get_name())
                }
            }
            // continue parsing child
            MatchResult::Continue(index, state) => {
                let mut expr_index = self.exprs.vec.len();
                let name = state.get_name();
                // replace none exprs
                if self.exprs[expr_index - 1].is_none() {
                    self.exprs.vec.pop();
                    expr_index -= 1;
                }
                self.exprs.vec.push(Expr::NoneExpr);
                self.stack.push((expr_index, index, state));

                self.last_result = LastMatchResult::None;

                ParserResult::Continue(name)
            }
            // I failed but could parse on future words
            MatchResult::ContinueFail => {
                let stack_index = self.stack.len() - 1;
                let frame = &mut self.stack[stack_index];
                // change match starting location to after word
                frame.1 = rest.pos;

                self.last_result = LastMatchResult::Continue;

                ParserResult::ContinueFail(frame.2.get_name())
            }
            // I failed and will not parse on future words
            MatchResult::Failed => {
                let frame = self.stack.pop().unwrap();

                let next_expr = self.stack.last().map_or(0, |x| x.0);
                // if state has not been replaced - remove from arena
                if !frame.2.do_replace() {
                    self.exprs.vec.truncate(next_expr);
                }

                self.last_result = LastMatchResult::Failed;
                // failed final stat - couldn't parse anything on line
                if self.stack.is_empty() {
                    self.parsing_line = false;
                    ParserResult::FailedLine(frame.2.get_name())
                } else {
                    // setup result for next step
                    ParserResult::Failed(frame.2.get_name())
                }
            }
        }
    }
}
