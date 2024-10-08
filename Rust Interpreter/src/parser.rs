#![allow(dead_code)]
#[path = "parser_source.rs"]
pub(crate) mod parser_source;
pub(crate) use parser_source::*;
// other stucts
#[path = "parser_structs.rs"]
pub(crate) mod parser_structs;
pub(crate) use parser_structs::*;
use rangemap::RangeSet;

mod basic_func;

mod alias;
pub(crate) mod alias_data;
mod append;
mod assign;
mod call_func;
mod color;
mod delete;
mod else_stat;
mod fill;
mod find;
mod foreach;
mod function;
mod if_stat;
mod index;
mod len;
mod list;
mod litcolor;
mod litcolor_data;
mod move_to;
mod not;
mod operator;
mod replace;
pub(crate) mod string_lit;
mod stroke;
mod var;
mod while_stat;

mod bezier;
mod circle;
mod line;
mod print;
mod rect;
mod return_stat;
mod rotate;

mod line_width;
pub(crate) mod multi_lit_num;
mod num_literal;
mod word_num;

#[path = "testing/parsing_tests_simple.rs"]
mod parsing_tests_simple;

// #[path = "testing/parsing_tests_milo.rs"]
// mod parsing_tests_milo;

// #[path = "testing/parsing_tests_other.rs"]
// mod parsing_tests_other;

use std::{collections::HashMap, fmt::Debug, mem};

use crate::{commands::*, writers::lisp_like_writer};

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
    //the set of current functions
    pub funcs: FuncSet,
    /// the ignored values
    pub nots: IgnoreSet,
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
    /// the index of the last matched state
    stat_indexes: Vec<usize>,
    /// the hash map of ranges of failed exprs
    cached_fails: HashMap<&'static str, RangeSet<usize>>,
}

impl<'a> Parser<'a> {
    ///make a new parser with a source and command flags
    pub fn new(source: ParserSource<'a>, flags: ParserFlags) -> Self {
        Parser {
            data: ParsedData {
                exprs: ExprArena { vec: Vec::new() },
                stat_starts: Vec::new(),
                vars: VarSet::new(),
                funcs: FuncSet::new(),
                nots: IgnoreSet::new(),
                source,
            },
            stack: Vec::new(),
            last_state: None,
            pos: 0,
            parsing_line: false,
            last_result: LastMatchResult::None,
            aliases: AliasData::new(flags),
            repeat_count: 0,
            stat_indexes: Vec::new(),
            cached_fails: HashMap::new(),
        }
    }
    ///get the last state
    pub fn get_last_state<'b>(&'b self) -> Option<&'b State> {
        self.last_state.as_ref().or_else(|| self.stack.last())
    }

    ///get the name of the last state
    pub fn get_last_state_name(&self) -> &'static str {
        self.get_last_state()
            .map_or(&"None", |state| state.state.get_name())
    }

    ///get the current stack and length
    pub fn get_parser_stack(&self) -> (String, usize) {
        let mut str = self.stack.iter().fold(String::new(), |mut str, state| {
            str += &format!("{}:{}, ", state.state.get_name(), state.last_parse);
            str
        });
        str.pop();
        str.pop();
        (str, self.stack.len())
    }

    ///get the slice that was last used + index
    pub fn get_last_word<'b>(&'b self) -> (&'b [u8], usize) {
        // (if self.last_result == LastMatchResult::Failed {
        //     self.get_last_state()
        // } else {
        //     self.stack.last()
        // })
        self.get_last_state().map_or((b"", usize::MAX), |state| {
            (
                Self::get_slice(self.data.source.get_line(), state.last_parse)
                    .0
                    .str,
                state.last_parse,
            )
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
        let _debug = only_debug!(Vec::from_iter(
            self.stack.iter().map(|x| (x.expr_index, x.last_parse))
        ));
        let _debug2 = only_debug!(Vec::from_iter(
            self.stack.iter().map(|x| x.state.get_name())
        ));
        let _expr = only_debug!(self.data.exprs.vec);
        let _expr2 = only_debug!(lisp_like_writer::write(
            &self.data.exprs,
            &self.data.stat_starts
        ));
        let _expr_short = only_debug!(Vec::from_iter(self.data.exprs.vec.iter().map(|e| {
            let mut str = format!("{:?}", e);
            str.truncate(str.find(" ").unwrap_or(str.len()));
            str
        })));
        let _last = only_debug!(self.last_result);

        self.last_state = None;
        // get curr frame
        let stack_index = self.stack.len() - 1;
        let (parents, frame_arr) = self.stack.split_at_mut(stack_index);
        let frame = &mut frame_arr[0];

        let id = frame.state.get_name();
        // does the failing range of the state include the current parsing location
        let must_fail = self
            .cached_fails
            .get(id)
            .is_some_and(|range| range.contains(&frame.last_parse));

        if must_fail && !cfg!(feature = "no-cache") {
            self.failed_func();
            return ParserResult::CachedFail;
        }
        // should always be in bounds
        // spilt at mut for borrow safety
        // get (parents, this[0] and children[1..])
        let parents_this = self.data.exprs.vec.split_at_mut(frame.expr_index);
        //let _splits1 = format!("{:?}", parents_this);
        // get (this, children)
        let this_children = parents_this.1.split_at_mut_checked(1);
        //let _splits = format!("{:?} {:?}", _splits1, this_children);

        // default_expr is used on failing back to a none state,
        // the corrisponding expr no longer exists
        let mut expr = &mut Expr::NoneExpr;
        let mut after: &mut [Expr] = &mut [];
        let before = parents_this.0;
        if let Some(split) = this_children {
            //should always be safe
            expr = split.0.first_mut().unwrap();
            after = split.1;
        }

        // let _self_expr = format!("{:?}", expr);
        //black_box(&_debug);
        //let mut last_stat = None;

        // // // if last expr matched
        // if let Some(index) = self.last_match_index {
        //     // let last_stat_index = self.data.exprs[last_stat];
        //     last_stat = split1.0.get_mut(index);
        // }

        // setup slice
        let line = self.data.source.get_line();
        let mut start = frame.last_parse;
        let (mut word, mut rest) = Self::get_slice(line, start);

        //New ignore code location
        while self.data.nots.try_get_val(&word, 0).is_some() {
            start = rest.pos;
            (word, rest) = Self::get_slice(line, start);
        }

        // setup env
        let mut env = Environment {
            expr,
            parents,
            after,
            before,
            last_stat_index: self.stat_indexes.last().cloned(),
            expr_index: frame.expr_index,
            vars: &mut self.data.vars,
            funcs: &mut self.data.funcs,
            nots: &mut self.data.nots,
            locs: None,
            global_index: self.pos,
            aliases: &self.aliases,
            full_text: line,
        };

        let last_result = mem::replace(&mut self.last_result, LastMatchResult::None);

        // run step function
        let mut result = match last_result {
            LastMatchResult::None | LastMatchResult::Continue => {
                frame.state.step(&mut env, &word, &rest)
            }
            LastMatchResult::New(locs) => {
                env.locs = locs;
                frame.state.step(&mut env, &word, &rest)
            }
            LastMatchResult::Matched(child_index) => {
                frame
                    .state
                    .step_match(&mut env, Some(child_index), &word, &rest)
            }
            LastMatchResult::Failed => frame.state.step_match(&mut env, None, &word, &rest),
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

        let state_type = state.state.get_type();
        if state_type == StateType::Stat {
            // remove self
            while self
                .stat_indexes
                .last()
                .is_some_and(|index| *index > state.expr_index)
            {
                self.stat_indexes.pop();
            }
        } else {
            //insert the range of parsed words into map
            let id = state.state.get_name();
            self.cached_fails
                .entry(id)
                .or_insert(RangeSet::new())
                .insert(state.first_parse..state.last_parse + 1);
        }

        let state_pos = state.expr_index;
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
        frame.last_parse = new_index;

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
        self.stack.push(State {
            expr_index,
            first_parse: index,
            last_parse: index,
            state,
        });

        self.last_result = LastMatchResult::New(locs);

        ParserResult::ContinueWith
    }

    ///this function is called if the step matches
    fn matched_func(&mut self, mut index: usize, closed: bool) -> ParserResult {
        let state = self.stack.pop().unwrap();
        let expr_index = state.expr_index;
        if state.state.get_type() == StateType::Stat {
            // add self
            self.stat_indexes.push(state.expr_index);
            // stats can change parse ablility -- reset cached fails
            self.cached_fails = HashMap::new();
        }
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
                let cd = get_close_data(&line[index..]);
                if self.repeat_count >= cd.close_count {
                    index += cd.close_length as usize;
                    self.repeat_count = 0;
                }
            }
            // setup result for next step
            self.last_result = LastMatchResult::Matched(expr_index);
            let parent_state = self.stack.last_mut().unwrap();
            parent_state.last_parse = index;

            // remove parent expr from cachefail map
            self.cached_fails.remove(parent_state.state.get_name());
            ParserResult::Matched
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
        let expr_index = self.data.exprs.vec.len();

        self.data.exprs.vec.push(Expr::NoneStat);

        self.stack.push(State {
            expr_index,
            first_parse: new_index,
            last_parse: new_index,
            state: Box::new(alias::NoneState::new_stat_cont()),
        });
        self.data.stat_starts.push(expr_index);
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
