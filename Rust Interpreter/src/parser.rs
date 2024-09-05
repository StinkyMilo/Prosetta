#![allow(dead_code)]
#[path = "parser_source.rs"]
pub(crate) mod parser_source;
pub(crate) use parser_source::*;
// other stucts
#[path = "parser_structs.rs"]
pub(crate) mod parser_structs;
pub(crate) use parser_structs::*;

mod basic_func;

mod alias;
pub(crate) mod alias_data;
mod assign;
mod not;
mod operator;
mod var;
mod word_num;
mod ifstatement;
mod whilestatement;

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

use crate::{commands::*, writers::linq_like_writer};

use alias_data::AliasData;

///The data that is currently parsed
#[derive(Debug)]
pub struct ParsedData<'a> {
    ///the array of parsed exprs
    pub exprs: ExprArena,
    ///the start indexes of statements
    pub stat_starts: Vec<usize>,
    ///the set of current varibles
    pub vars: VarSet,
    ///the parserSource that is used
    pub source: ParserSource<'a>,
}

#[derive(Debug)]
pub struct Parser<'a> {
    ///the currently parsed data
    pub data: ParsedData<'a>,
    ///the stack of states
    stack: Vec<State>,
    ///the last state that was dropped (if the last step Matched or Failed)
    last_state: Option<State>,
    ///is the parser currently paring or line or did last step end (see ParserResult::is_end)
    parsing_line: bool,
    ///the global position without respect to ParserSource buffers
    pos: usize,
    ///the last match result
    last_result: LastMatchResult,
    ///the static alias data
    aliases: AliasData,
    ///the number of times the current slice should repeat
    repeat_count: u8,
}

impl<'a> Parser<'a> {
    ///make a new parser with a source and command flags
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
            repeat_count: 0,
        }
    }
    ///get the last state
    pub fn get_last_state<'b>(&'b self) -> Option<&'b State> {
        self.last_state.as_ref().or_else(|| self.stack.last())
    }

    ///get the name of the last state
    pub fn get_last_state_name(&self) -> &'static str {
        self.get_last_state()
            .map_or(&"None", |state| state.2.get_name())
    }

    ///get the slice that was last used
    pub fn get_last_word<'b>(&'b self) -> &'b [u8] {
        // (if self.last_result == LastMatchResult::Failed {
        //     self.get_last_state()
        // } else {
        //     self.stack.last()
        // })
        self.get_last_state().map_or(b"", |state| {
            Self::get_slice(self.data.source.get_line(), state.1).0.str
        })
    }

    ///convert the parser into its data
    ///also drops the stdin from the ParserSource
    pub fn into_data(mut self) -> ParsedData<'a> {
        self.data.source.drop_input();
        self.data
    }
}

///the parser - Woah!!
impl<'a> Parser<'a> {
    ///step the parser
    ///
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
        //debug time
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
        let mut env = Environment {
            exprs: &mut self.data.exprs,
            index: frame.0,
            vars: &mut self.data.vars,
            locs: None,
            global_index: self.pos,
            aliases: &self.aliases
        };

        // setup slice
        let line = self.data.source.get_line();
        let (word, rest) = Self::get_slice(line, frame.1);

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
            MatchResult::Matched(index, bool) => self.matched_func(index, bool),
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

    ///this function is called if the step fails
    fn failed_func(&mut self) -> ParserResult {
        let state = self.stack.pop().unwrap();

        let state_pos = state.0;
        self.data.exprs.vec.truncate(state_pos);
        //let _test = format!("{:?}", state);
        self.repeat_count = 0;

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

    ///this function is called if the step coninues
    fn continue_func(&mut self, new_index: usize) -> ParserResult {
        let stack_index = self.stack.len() - 1;
        let frame = &mut self.stack[stack_index];

        self.repeat_count = 0;
        // change match starting location to after word
        frame.1 = new_index;

        self.last_result = LastMatchResult::Continue;

        ParserResult::Continue
    }
    ///this function is called if the step coninues with
    fn continue_with_func(
        &mut self,
        index: usize,
        state: Box<dyn ParseState>,
        locs: Option<Vec<usize>>,
    ) -> ParserResult {
        let mut expr_index = self.data.exprs.vec.len();
        self.repeat_count = 0;
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

    ///this function is called if the step matches
    fn matched_func(&mut self, mut index: usize, closed: bool) -> ParserResult {
        let state = self.stack.pop().unwrap();
        let expr_index = state.0;
        self.last_state = Some(state);

        // matched final stat
        if self.stack.is_empty() {
            // setup next
            self.add_new_nonestat(index);
            ParserResult::MatchedLine
        } else {
            if closed {
                let line = self.data.source.get_line();
                self.repeat_count += 1;
                // get needed counts
                let (needed_count, offset) = Self::get_repeat_count(index, line);
                if self.repeat_count >= needed_count {
                    index += offset as usize;
                    self.repeat_count = 0;
                }
            }
            // setup result for next step
            self.last_result = LastMatchResult::Matched(expr_index);
            self.stack.last_mut().unwrap().1 = index;
            ParserResult::Matched
        }
    }
    /// gets the number of times the characters at line[index] should be repeated and the offset after
    /// returns (repeat_count,offset)
    fn get_repeat_count(index: usize, line: &[u8]) -> (u8, u8) {
        if line[index..index + 2] == b"..."[..] {
            (255, 3)
        } else {
            (
                match line[index] {
                    b'.' | b',' | b':' => 1,
                    b'?' | b'!' => 2,
                    _ => 0,
                },
                1,
            )
        }
    }

    ///get a (word,rest) that starts at start
    fn get_slice(line: &[u8], mut start: usize) -> (Slice, Slice) {
        //let line = line.as_bytes();
        start = start.min(line.len());

        let slice = Slice {
            str: &line[start..],
            pos: start,
        };
        get_next_slice(&slice, 0)
    }
    ///setup the a new line for parsing
    fn setup_first(&mut self) -> bool {
        let line = self.data.source.get_line();
        self.pos += line.len();
        let data = self.data.source.new_line();
        if let Some(data) = data {
            let found_data = trim_ascii_whitespace(data).len() > 0;
            if found_data {
                self.add_new_nonestat(0);
                self.parsing_line = true;
            }
            found_data
        } else {
            false
        }
    }
    ///setup a noneStat on the stack
    fn add_new_nonestat(&mut self, new_index: usize) {
        // push match stat on first step of line
        let index = self.data.exprs.vec.len();

        self.data.exprs.vec.push(Expr::NoneStat);

        self.stack
            .push((index, new_index, Box::new(alias::NoneState::new_stat())));
        self.data.stat_starts.push(index);
        self.last_result = LastMatchResult::None;
    }
}

/// https://stackoverflow.com/questions/31101915/how-to-implement-trim-for-vecu8
pub fn trim_ascii_whitespace(x: &[u8]) -> &[u8] {
    let from = match x.iter().position(|x| !x.is_ascii_whitespace()) {
        Some(i) => i,
        None => return &x[0..0],
    };
    let to = x.iter().rposition(|x| !x.is_ascii_whitespace()).unwrap();
    &x[from..=to]
}
