#![allow(dead_code)]

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
type BuildInSetUp = fn(num: u16, index: usize, child_index: usize, locs: Vec<usize>) -> MatchResult;
type StepFunction =
    fn(env: &mut Environment, result: MatchChildResult, word: &Slice, rest: &Slice) -> MatchResult;

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

#[derive(PartialEq, Debug)]
enum StateContext {
    None,
    MultStep { step_num: u8 },
    Matching(BuiltinMatchState),
}
impl Default for StateContext {
    fn default() -> Self {
        StateContext::None
    }
}
impl StateContext {
    pub fn get_name(&self) -> &'static str {
        match self {
            StateContext::None => "NoState",
            StateContext::MultStep { .. } => "MultStep",
            StateContext::Matching(BuiltinMatchState { is_expr, .. }) => {
                if *is_expr {
                    Expr::NoneExpr.get_name()
                } else {
                    Expr::NoneStat.get_name()
                }
            }
        }
    }
    pub fn is_none_expr_state(&self) -> bool {
        match self {
            StateContext::Matching(..) => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Debug)]
enum MatchResult {
    Matched(usize),
    Continue(usize, Expr, StateContext),
    ContinueFail,
    Failed,
}
#[derive(PartialEq, Debug, Clone, Copy)]
enum MatchChildResult {
    None,
    Matched,
    Failed,
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

struct Environment<'a> {
    vars: &'a VarSet,
    expr: &'a mut Expr,
    state: &'a mut StateContext,
    child_index: usize,
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
    // (expr_index, string_index)
    stack: Vec<(usize, usize)>,
    stack_state: Vec<StateContext>,

    pub vars: VarSet,

    curr_line: String,
    source: &'a mut dyn ParseSource,
    parsing_line: bool,

    pos: usize,
    last_result: MatchChildResult,
}

type ParseFunc = fn() -> MatchResult;
impl<'a> Parser<'a> {
    pub fn new(source: &'a mut dyn ParseSource) -> Self {
        Parser {
            exprs: ExprArena { vec: Vec::new() },
            stack: Vec::new(),
            stack_state: Vec::new(),
            vars: VarSet::new(),
            stat_starts: Vec::new(),
            curr_line: String::new(),
            pos: 0,
            source,
            parsing_line: false,
            last_result: MatchChildResult::None,
        }
    }
    pub fn change_source(&mut self, source: &'a mut dyn ParseSource) {
        self.source = source;
    }

    fn setup_first(&mut self) -> bool {
        self.curr_line = String::new();
        let has_read = self.source.read_line(&mut self.curr_line).is_ok();
        if has_read {
            // push match stat on first step of line
            let index = self.exprs.vec.len();

            self.exprs.vec.push(Expr::NoneStat);

            self.stack.push((index, self.pos));
            self.stack_state.push(StateContext::None);
            self.stat_starts.push(index);

            self.parsing_line = true;
        }
        has_read
    }

    pub fn step(&mut self) -> ParserResult {
        let is_first = !self.parsing_line;
        if is_first {
            // push match stat on first step of line
            if !self.setup_first() {
                return ParserResult::NoInput;
            }
        }
        let _debug = format!("{:?}", self.stack);
        let _debug2 = format!("{:?}", self.stack_state);
        let _expr = format!("{:?}", self.exprs.vec);
        let _expr2 = linq_like_writer::write(&self.exprs, &self.stat_starts);
        black_box(&_debug);
        black_box(&_debug2);
        black_box(&_expr);
        black_box(&_expr2);
        // get curr frame
        let stack_index = self.stack.len() - 1;
        let stack = self.stack[stack_index];
        let mut expr = &mut Expr::NoneExpr;

        let next_child = self.exprs.vec.len();

        if stack.0 < self.exprs.vec.len() {
            expr = &mut self.exprs.vec[stack.0];
        }
        let frame = &mut self.stack_state[stack_index];
        // setup env
        let mut env = Environment {
            expr,
            state: frame,
            vars: &self.vars,
            child_index: next_child,
        };

        let line = self.curr_line.as_bytes();
        let start = stack.1 - self.pos;
        let slice = Slice {
            str: &line[start..],
            pos: start,
        };

        let (word,rest) = get_next_word(&slice, 0);
        //let mut word_end = 0;
        // run step function
        let result;
        // if let Some((word, rest)) = words {
            let func = get_step_fn(&env);
            result = func(&mut env, self.last_result, &word, &rest);
            //word_end = rest.pos;
        // } else {
        //     result = MatchResult::Failed;
        // }

        // run aftermath

        match result {
            // I matched - return to last expr on stack with success
            MatchResult::Matched(index) => {
                let expr_index = self.stack.pop().unwrap().0;
                let state = self.stack_state.pop().unwrap();
                let expr=&self.exprs[expr_index];
                // matched final stat
                if self.stack.is_empty() {
                    self.parsing_line = false;
                    ParserResult::MatchedLine(get_name_from_state(expr,&state))
                } else {
                    // setup result for next step
                    self.last_result = MatchChildResult::Matched;
                    self.stack[stack_index - 1].1 = index;
                    ParserResult::Matched(get_name_from_state(expr,&state))
                }
            }
            // continue parsing child
            MatchResult::Continue(index, new_expr, state) => {
                let mut expr_index = self.exprs.vec.len();
                let name = new_expr.get_name();
                // replace none exprs
                if self.exprs[expr_index - 1].is_none() {
                    self.exprs.vec.pop();
                    expr_index -= 1;
                }
                self.exprs.vec.push(new_expr);
                self.stack.push((expr_index, index));
                self.stack_state.push(state);
                ParserResult::Continue(name)
            }
            // I failed but could parse on future words
            MatchResult::ContinueFail => {
                let stack_index = self.stack.len() - 1;
                let expr_index = &mut self.stack[stack_index];
                // change match starting location to after word
                expr_index.1 = rest.pos;

                ParserResult::ContinueFail(self.exprs[expr_index.0].get_name())
            }
            // I failed and will not parse on future words
            MatchResult::Failed => {
                let expr_index = self.stack.pop().unwrap().0;
                self.stack_state.pop();

                let next_expr = self.stack.last().map_or(0, |x| x.0);
                self.exprs.vec.truncate(next_expr);
                // failed final stat - couldn't parse anything on line
                if self.stack.is_empty() {
                    self.parsing_line = false;
                    ParserResult::FailedLine(self.exprs[expr_index].get_name())
                } else {
                    // setup result for next step
                    self.last_result = MatchChildResult::Failed;
                    ParserResult::Failed(self.exprs[expr_index].get_name())
                }
            }
        }
    }
}

fn get_name_from_state(expr:&Expr,state:&StateContext)->&'static str{
    if state.is_none_expr_state() {
        state.get_name()
    } else {
        expr.get_name()
    }
}

// functions
fn get_step_fn(env: &Environment) -> StepFunction {
    match &*env.state {
        StateContext::Matching(BuiltinMatchState { is_expr, .. }) => {
            if *is_expr {
                step_expr
            } else {
                step_stat
            }
        }
        state => match &*env.expr {
            Expr::NoneStat => step_stat,
            Expr::NoneExpr => step_expr,
            Expr::Num { .. } => step_num,
            Expr::Eq { .. } => step_eq,
            Expr::Var { .. } => step_var,
            Expr::Mult { .. } => step_mult,
            Expr::Add { .. } => step_add,
            // (Expr::Add { .. }, _) => step_add,
            // (Expr::Mult { .. }, _) => step_mult,
            expr => panic!("State ({expr:?},{state:?}) should not happen"),
        },
    }
}

fn step_stat(
    env: &mut Environment,
    result: MatchChildResult,
    word: &Slice,
    rest: &Slice,
) -> MatchResult {
    match_built_in(&STAT_DATA, env, result, word, rest)
}
fn step_expr(
    env: &mut Environment,
    result: MatchChildResult,
    word: &Slice,
    rest: &Slice,
) -> MatchResult {
    if result == MatchChildResult::None {
        let var_result = step_var(env, result, word, rest);
        if matches!(var_result, MatchResult::Matched(_)) {
            return var_result;
        }
    }

    match_built_in(&EXPR_DATA, env, result, word, rest)
}

fn step_eq(
    env: &mut Environment,
    result: MatchChildResult,
    word: &Slice,
    rest: &Slice,
) -> MatchResult {
    match result {
        // first time
        MatchChildResult::None => {
            let Expr::Eq {
                name_start,
                name,
                value_index,
                ..
            } = env.expr
            else {
                unimplemented!()
            };
            *name_start = word.pos;
            *name = word.str.to_owned();
            *value_index = env.child_index;
            // setup expr child
            MatchResult::Continue(rest.pos, Expr::NoneExpr, StateContext::None)
        }
        // child expr matched
        MatchChildResult::Matched => {
            let close = find_h_close(&word, 0).or_else(|| find_h_close(&rest, 0));
            match close {
                // will never be a h to find even on future words
                None => MatchResult::Failed,
                Some(slice) => {
                    // let Expr::Eq {  .. } = env.expr else {
                    //     unimplemented!()
                    // };
                    MatchResult::Matched(slice.pos)
                }
            }
        }
        // child expr failed
        // if child match fail, I can never succeed
        MatchChildResult::Failed => MatchResult::Failed,
    }
}

fn step_var(
    env: &mut Environment,
    _result: MatchChildResult,
    word: &Slice,
    rest: &Slice,
) -> MatchResult {
    // is varible in scope
    if env.vars.contains(word.str) {
        *env.expr = Expr::Var {
            name_start: word.pos,
            name: word.str.to_owned(),
        };
        // *name_start = word.pos;
        // *name = word.str.to_owned();
        MatchResult::Matched(rest.pos)
    } else {
        // future words could be varible names
        MatchResult::ContinueFail
    }
}

fn step_num(
    env: &mut Environment,
    _result: MatchChildResult,
    word: &Slice,
    rest: &Slice,
) -> MatchResult {
    let close = find_h_close(rest, 0);
    if let Some(close) = close {
        let Expr::Num { str_start, str, .. } = env.expr else {
            unimplemented!()
        };

        *str_start = word.pos;
        *str = word.str.to_owned().to_ascii_lowercase();
        return MatchResult::Matched(close.pos);
    }

    // will not work on next word
    MatchResult::Failed
}

#[derive(Debug, PartialEq)]
pub struct BuiltinData {
    names: &'static [&'static [u8]],
    func: BuildInSetUp,
    is_expr: bool,
}
const EXPR_COMS: [&'static [u8]; 3] = ["num".as_bytes(), "mu".as_bytes(), "and".as_bytes()];
const STAT_COMS: [&'static [u8]; 3] = ["eq".as_bytes(), "pi".as_bytes(), "li".as_bytes()];

const EXPR_DATA: BuiltinData = BuiltinData {
    names: &["num".as_bytes(), "mu".as_bytes(), "and".as_bytes()],
    func: setup_expr,
    is_expr: true,
};

fn setup_expr(num: u16, index: usize, child_index: usize, locs: Vec<usize>) -> MatchResult {
    MatchResult::Continue(
        index,
        match num {
            0 => Expr::Num {
                locs,
                str_start: 0,
                str: Vec::new(),
            },
            1 => Expr::Mult {
                locs,
                a_index: child_index,
                b_index: usize::MAX,
            },
            2 => Expr::Add {
                locs,
                a_index: child_index,
                b_index: usize::MAX,
            },
            _ => unimplemented!(),
        },
        StateContext::None,
    )
}

const STAT_DATA: BuiltinData = BuiltinData {
    names: &["eq".as_bytes(), "pi".as_bytes(), "li".as_bytes()],
    func: setup_stat,
    is_expr: false,
};

fn setup_stat(num: u16, index: usize, child_index: usize, locs: Vec<usize>) -> MatchResult {
    MatchResult::Continue(
        index,
        match num {
            0 => Expr::Eq {
                locs,
                name_start: 0,
                name: Vec::new(),
                value_index: child_index,
            },
            1 => Expr::Circle {
                locs,
                x_index: child_index,
                y_index: usize::MAX,
                r_index: usize::MAX,
            },
            2 => Expr::Line {
                locs,
                x_index: child_index,
                y_index: usize::MAX,
                x2_index: usize::MAX,
                y2_index: usize::MAX,
            },
            _ => unimplemented!(),
        },
        StateContext::None,
    )
}

fn match_built_in(
    data: &BuiltinData,
    env: &mut Environment,
    result: MatchChildResult,
    word: &Slice,
    rest: &Slice,
) -> MatchResult {
    let length = data.names.len();
    debug_assert!(length < u16::MAX as usize);

    //is child matched
    match result {
        // child matched successfully
        MatchChildResult::Matched => return MatchResult::Matched(word.pos),
        _ => {}
    }

    // use or setup state
    // let mut context_state = mem::take(env.state);

    // let state = match &mut context_state {
    //     StateContext::Matching(state) => state,
    //     none_state => {
    //         let new_state = BuiltinMatchState {
    //             progress: vec![0u8; length],
    //             locs: vec![Some(Vec::new()); length],
    //             offset: 0,
    //             matched: 0,
    //             is_expr: data.is_expr,
    //         };

    //         // rust trickery - probably better way to do this
    //         *none_state = StateContext::Matching(new_state);
    //         if let StateContext::Matching(new_state) = none_state {
    //             new_state
    //         } else {
    //             unimplemented!()
    //         }
    //     }
    // };
    let context_state = &mut env.state;

    let state = match context_state {
        StateContext::Matching(state) => state,
        none_state => {
            let new_state = BuiltinMatchState {
                progress: vec![0u8; length],
                locs: vec![Some(Vec::new()); length],
                offset: 0,
                matched: 0,
                is_expr: data.is_expr,
            };

            // rust trickery - probably better way to do this
            **none_state = StateContext::Matching(new_state);
            if let StateContext::Matching(new_state) = none_state {
                new_state
            } else {
                unimplemented!()
            }
        }
    };

    // run until end of word
    for offset in state.offset..word.len() {
        // does letter match any commands
        for i in 0..length {
            // does letter match
            if state.progress[i] < data.names[i].len() as u8
                && word.str[offset].to_ascii_lowercase()
                    == data.names[i][state.progress[i] as usize]
            {
                state.progress[i] += 1;
                // add locations to locations (locs)
                state.locs[i].as_mut().unwrap().push(word.pos + offset);
                if state.progress[i] == data.names[i].len() as u8 {
                    state.matched += 1;
                }
            }
        }

        // try match
        while state.matched != 0 {
            state.matched -= 1;
            let mut min_size = usize::MAX;
            let mut min_locations = usize::MAX;
            let mut min_index = u16::MAX;
            for j in 0..length {
                // has finished matching
                if state.progress[j] == data.names[j].len() as u8 {
                    let matching_locs = state.locs[j].as_ref().unwrap();

                    let size = matching_locs.last().unwrap() - matching_locs[0];
                    let location_sum: usize = matching_locs.iter().sum();

                    // is best match
                    if size < min_size || (size == min_size && location_sum < min_locations) {
                        min_index = j as u16;
                        min_size = size;
                        min_locations = location_sum;
                    }
                }
            }
            state.offset = offset;

            //set up stack
            return (data.func)(
                min_index,
                rest.pos,
                env.child_index,
                // move locs out of state without copy
                state.locs[min_index as usize].take().unwrap(),
            );
        }
    }
    // try match next word
    MatchResult::ContinueFail
}

fn step_bi_fn(
    env: &mut Environment,
    result: MatchChildResult,
    word: &Slice,
    rest: &Slice,
) -> MatchResult {
    //match result {}
    MatchResult::Failed
}

fn step_add(
    env: &mut Environment,
    result: MatchChildResult,
    word: &Slice,
    rest: &Slice,
) -> MatchResult {
    step_bi_fn(env, result, word, rest)
}
fn step_mult(
    env: &mut Environment,
    result: MatchChildResult,
    word: &Slice,
    rest: &Slice,
) -> MatchResult {
    step_bi_fn(env, result, word, rest)
}
