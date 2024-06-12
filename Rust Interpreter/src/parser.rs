#![allow(dead_code)]

mod add_mult;
mod alias;
pub(crate) mod alias_data;
mod eq;
mod num;
mod var;

mod circle;
mod line;

mod num_lit;
mod num_literal;

#[path = "parsing_tests_word_funcs.rs"]
mod parsing_tests_word_funcs;

#[path = "parsing_tests_milo.rs"]
mod parsing_tests_milo;

#[path = "parsing_tests_other.rs"]
mod parsing_tests_other;

use std::{
    collections::HashSet,
    fmt::{self, Debug},
    hint::black_box,
    io::BufRead,
};

use crate::{commands::*, linq_like_writer};

use alias_data::AliasData;

//type MatchResult<T> = Option<(usize, T)>;
type VarSet = HashSet<Vec<u8>>;
//type ParseFn = fn(this: &Parser,&VarSet, &Slice<'_>, Vec<usize>) -> MatchResult;
type StepFunction =
    fn(env: &mut Enviroment, result: LastMatchResult, word: &Slice, rest: &Slice) -> MatchResult;

pub trait ParseSource: BufRead + Debug {}
impl<T: BufRead + Debug> ParseSource for T {}

#[derive(Default, Debug)]
pub struct ParserFlags {
    pub not: bool,
}

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
pub trait ParseState: Debug {
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
pub enum MatchResult {
    Matched(usize),
    ContinueWith(usize, Box<dyn ParseState>),
    Continue,
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
    Start,

    MatchedLine,
    FailedLine,

    Matched,
    ContinueWith,
    Continue,
    Failed,
}

pub struct Enviroment<'a>{
    pub vars: &'a VarSet,
    pub expr: &'a mut Expr,
    pub locs: Option<Vec<usize>>,
    pub child_index: usize,
    pub global_index: usize,
    pub aliases: &'a AliasData,
}

#[derive(PartialEq)]
pub struct Slice<'a> {
    pub str: &'a [u8],
    pub pos: usize,
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

fn is_valid_word_char(char: u8) -> bool {
    char.is_ascii_alphanumeric() || char == b'-'
}

fn get_next_word<'a>(slice: &Slice<'a>, mut start: usize) -> (Slice<'a>, Slice<'a>) {
    // find start of word
    start = start.min(slice.len());
    while start < slice.len() && !is_valid_word_char(slice.str[start]) {
        start += 1;
    }

    // find end of word
    let mut end = start;
    while end < slice.len() && is_valid_word_char(slice.str[end]) {
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
    while end < slice.len() && is_valid_word_char(slice.str[end]) {
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

// (expr_index, string_index, state)
pub type State = (usize, usize, Box<dyn ParseState>);

#[derive(Debug)]
pub struct Parser<'a> {
    pub exprs: ExprArena,
    pub stat_starts: Vec<usize>,

    stack: Vec<State>,
    last_state: Option<State>,

    pub vars: VarSet,

    curr_line: Vec<u8>,
    source: &'a mut dyn ParseSource,
    parsing_line: bool,

    pos: usize,
    last_result: LastMatchResult,
    last_locs: Option<Vec<usize>>,
    aliases: AliasData,
}

type ParseFunc = fn() -> MatchResult;
impl<'a> Parser<'a> {
    pub fn new(source: &'a mut dyn ParseSource, flags: ParserFlags) -> Self {
        Parser {
            exprs: ExprArena { vec: Vec::new() },
            stack: Vec::new(),
            last_state: None,
            vars: VarSet::new(),
            stat_starts: Vec::new(),
            curr_line: Vec::new(),
            pos: 0,
            source,
            parsing_line: false,
            last_result: LastMatchResult::None,
            last_locs: None,
            aliases: AliasData::new(Default::default()),
        }
    }
    pub fn change_source(&mut self, source: &'a mut dyn ParseSource) {
        self.source = source;
    }

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
        self.get_last_state()
            .map_or(b"", |state| Self::get_slice(&self.curr_line, state.1).0.str)
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
        let _expr = format!("{:?}", self.exprs.vec);
        let _expr2 = linq_like_writer::write(&self.exprs, &self.stat_starts);
        let _expr_short = format!(
            "{:?}",
            self.exprs.vec.iter().map(|e| {
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
            aliases: &self.aliases,
        };

        // setup slice

        let (word, rest) = Self::get_slice(&self.curr_line, frame.1);

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
        if word.len() == 0 && matches!(result, MatchResult::Continue) {
            result = MatchResult::Failed;
        }

        match result {
            // I matched - return to last expr on stack with success
            MatchResult::Matched(index) => self.matched_func(index),
            // continue parsing child
            MatchResult::ContinueWith(index, state) => self.continue_with_func(index, state),
            // continue with me
            MatchResult::Continue => self.continue_func(rest.pos),
            // I failed, go back on stack with fail
            MatchResult::Failed => self.failed_func(),
        }
    }

    fn failed_func(&mut self) -> ParserResult {
        let state = self.stack.pop().unwrap();

        let state_pos = state.0;
        self.exprs.vec.truncate(state_pos);
        //let _test = format!("{:?}", state);

        self.last_state = Some(state);

        self.last_result = LastMatchResult::Failed;
        // failed final stat - couldn't parse anything on line
        if self.stack.is_empty() {
            self.parsing_line = false;
            self.stat_starts.pop();
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

    fn continue_with_func(&mut self, index: usize, state: Box<dyn ParseState>) -> ParserResult {
        let mut expr_index = self.exprs.vec.len();

        // replace none exprs
        if self.exprs.vec.last().is_some_and(|e| e.is_none()) {
            self.exprs.vec.pop();
            expr_index -= 1;
        }
        self.exprs.vec.push(Expr::NoneExpr);
        self.stack.push((expr_index, index, state));

        self.last_result = LastMatchResult::None;

        ParserResult::ContinueWith
    }

    fn matched_func(&mut self, index: usize) -> ParserResult {
        let state = self.stack.pop().unwrap();
        self.last_state = Some(state);

        // matched final stat
        if self.stack.is_empty() {
            let start_index = *self.stat_starts.last().unwrap();
            self.parsing_line = false;
            // add to varibles
            if let Expr::Eq { name, .. } = &self.exprs[start_index] {
                self.vars.insert(name.to_owned());
            }
            ParserResult::MatchedLine
        } else {
            // setup result for next step
            self.last_result = LastMatchResult::Matched;
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
        get_next_word(&slice, 0)
    }

    fn setup_first(&mut self) -> bool {
        self.pos += self.curr_line.len();
        self.curr_line = Vec::new();
        let has_read = self.source.read_until(b'\n', &mut self.curr_line).is_ok();
        let found_data = has_read && trim_ascii_whitespace(&self.curr_line).len() > 0;
        if found_data {
            // push match stat on first step of line
            let index = self.exprs.vec.len();

            self.exprs.vec.push(Expr::NoneStat);

            self.stack.push((
                index,
                0,
                Box::new(alias::NoneState::new_stat()),
            ));
            self.stat_starts.push(index);

            self.parsing_line = true;
            self.last_locs = None;
            self.last_result = LastMatchResult::None;
        }
        found_data
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
