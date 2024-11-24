#![allow(dead_code)]
#[path = "parser_source.rs"]
pub(crate) mod parser_source;
use alias::WordTriggerArena;
use bstr::ByteSlice;
pub(crate) use parser_source::*;
// other stucts
#[path = "parser_structs.rs"]
pub(crate) mod parser_structs;
pub(crate) use parser_structs::*;
#[path = "fail_map.rs"]
pub(crate) mod fail_map;
pub(crate) use fail_map::*;

pub(crate) mod alias;
pub(crate) mod alias_data;
mod append;
mod assign;
mod basic_func;
mod bezier;
mod call_func;
mod circle;
mod color;
mod comment;
mod delete;
mod else_stat;
mod fill;
mod find;
mod foreach;
mod frame;
mod function;
mod if_stat;
mod index;
mod len;
mod line;
mod line_width;
mod list;
mod litcolor;
mod litcolor_data;
mod move_to;
pub(crate) mod multi_lit_num;
mod not;
mod num_literal;
mod operator;
mod print;
mod rand;
mod rect;
mod replace;
mod return_stat;
mod rotate;
mod stamps;
pub(crate) mod string_lit;
mod stroke;
mod title;
mod trig;
mod var;
mod while_stat;
mod word_num;
mod floor;

#[path = "testing/mod.rs"]
mod testing;

use crate::commands::*;
use std::{collections::HashMap, fmt::Debug, mem};

use alias_data::AliasData;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Import {
    List,
    Func,
    Graph,
    Frame,
    Trig,
    Rand,
    Stamp,
}

impl Import {
    pub fn get_name(&self) -> &'static str {
        match self {
            Import::List => "List",
            Import::Func => "Func",
            Import::Graph => "Graph",
            Import::Frame => "Frame",
            Import::Trig => "Trig",
            Import::Rand => "Rand",
            Import::Stamp => "Stamp",
        }
    }
    pub fn get_all() -> &'static [(Import, &'static [u8])] {
        &[
            (Import::List, b"list"),
            (Import::Func, b"func"),
            (Import::Graph, b"graph"),
            (Import::Frame, b"fram"),
            (Import::Trig, b"trig"),
            (Import::Rand, b"rand"),
            (Import::Stamp, b"stamp"),
        ]
    }
}

#[derive(Debug, PartialEq)]
pub struct Title {
    pub title: Vec<u8>,
    // the imports: (name, position, length)
    pub authors: Vec<(Vec<u8>, usize, usize)>,
    // the imports: (type, position, length)
    pub imports: Vec<(Import, usize, u8)>,
    // the sepatators: (position, length)
    pub delim: Vec<(usize, u8)>,
    // the start of "by"
    pub by_start: usize,
}
impl Title {
    pub fn new() -> Self {
        Self {
            title: Vec::new(),
            authors: Vec::new(),
            imports: Vec::new(),
            delim: Vec::new(),
            by_start: usize::MAX,
        }
    }
}

///The data that is currently parsed
#[derive(Debug)]
pub struct ParsedData<'a> {
    ///the array of parsed exprs
    pub exprs: ExprArena,
    ///the start indexes of statements
    pub stat_starts: Vec<usize>,
    ///the set of current varibles
    pub symbols: SymbolSet,
    /// the ignored values
    pub nots: IgnoreSet,
    ///the parserSource that is used
    pub source: ParserSource<'a>,
    /// the imports
    pub imports: Vec<Import>,
    /// The global start and end of alias data
    pub trigger_word_data: WordTriggerArena,
    /// The number of steps taken to parse
    pub steps: u64,
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
    cached_fails: FailMap,
    ///does the parser need to parse a title
    parse_title: bool,
    //has the parser skipped a non Noneexpr state cache fail
    has_skipped_cache_fail: bool,
}

impl<'a> Parser<'a> {
    ///make a new parser with a source and command flags
    pub fn new(source: ParserSource<'a>, flags: ParserFlags) -> Self {
        // let title = (!flags.title).then(|| Title {
        //     title: b"No Title".to_vec(),
        //     author: vec![b"No Author".to_vec()],
        //     imports:
        // });
        let aliases = if flags.title {
            AliasData::none()
        } else {
            AliasData::all()
        };

        Parser {
            data: ParsedData {
                exprs: ExprArena { vec: Vec::new() },
                stat_starts: Vec::new(),
                symbols: SymbolSet::new(),
                nots: IgnoreSet::new(),
                imports: Vec::new(),
                source,
                trigger_word_data: WordTriggerArena::new(),
                steps: 1,
            },
            parse_title: flags.title,
            stack: Vec::new(),
            last_state: None,
            pos: 0,
            parsing_line: false,
            last_result: LastMatchResult::None,
            aliases,
            repeat_count: 0,
            stat_indexes: Vec::new(),
            cached_fails: FailMap::new(),
            has_skipped_cache_fail: false,
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
        // let _debug = only_debug!(Vec::from_iter(
        //     self.stack.iter().map(|x| (x.expr_index, x.last_parse))
        // ));
        // let _debug2 = only_debug!(Vec::from_iter(
        //     self.stack.iter().map(|x| x.state.get_name())
        // ));
        // let _expr = only_debug!(self.data.exprs.vec);
        // let _expr2 = only_debug!(lisp_like_writer::write(
        //     &self.data.exprs,
        //     &self.data.stat_starts
        // ));
        // let _expr_short = only_debug!(Vec::from_iter(self.data.exprs.vec.iter().map(|e| {
        //     let mut str = format!("{:?}", e);
        //     str.truncate(str.find(" ").unwrap_or(str.len()));
        //     str
        // })));
        // let _last = only_debug!(self.last_result);

        self.last_state = None;
        // get curr frame
        let stack_index = self.stack.len() - 1;
        let (parents, frame_arr) = self.stack.split_at_mut(stack_index);
        let frame = &mut frame_arr[0];

        // cache
        if !cfg!(feature = "no-cache") {
            // give states a one step buffer into fail territory:
            // noneexprs should always fail
            // states that just started always fail
            if frame.state.get_type() == StateType::None
                || matches!(self.last_result, LastMatchResult::New(..))
                || self.has_skipped_cache_fail
            {
                let id = frame.state.get_name();
                // does the failing range of the state include the current parsing location
                let must_fail = self
                    .cached_fails
                    .contains(id, frame.types, frame.last_parse);

                if must_fail {
                    self.failed_func(None);
                    return ParserResult::CachedFail;
                }
            } else {
                self.has_skipped_cache_fail = true;
            }
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
            symbols: &mut self.data.symbols,
            nots: &mut self.data.nots,
            locs: None,
            global_index: self.pos,
            aliases: &self.aliases,
            full_text: line,
            trigger_word_data: &mut self.data.trigger_word_data,
            types: frame.types,
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
            LastMatchResult::Matched(child_index, return_type) => {
                frame
                    .state
                    .step_match(&mut env, Some((child_index, return_type)), &word, &rest)
            }
            LastMatchResult::Failed => frame.state.step_match(&mut env, None, &word, &rest),
        };

        // run aftermath
        let new_locs = env.locs.take();

        // reached end of line - upgrade result to failed
        if word.len() == 0 && matches!(result, MatchResult::Continue(0)) {
            result = MatchResult::Failed;
        }

        match result {
            // I matched - return to last expr on stack with success
            MatchResult::Matched(index, return_type, bool) => {
                self.matched_func(index, return_type, bool)
            }
            // continue parsing child
            MatchResult::ContinueWith(index, types, state) => {
                self.continue_with_func(index, types, state, new_locs)
            }
            // continue with me
            MatchResult::Continue(index) => self.continue_func(rest.pos + index),
            // I failed, go back on stack with fail
            MatchResult::Failed => self.failed_func(Some(rest.pos)),
        }
    }

    ///this function is called if the step fails
    /// takes the postion of the rest of string if not cachedfailed
    fn failed_func(&mut self, rest_pos: Option<usize>) -> ParserResult {
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
        } else if let Some(index) = rest_pos {
            //insert the range of parsed words into map
            let id = state.state.get_name();
            self.cached_fails
                .insert(id, state.types, state.first_parse..index);
        }

        let state_pos = state.expr_index;
        self.data.exprs.vec.truncate(state_pos);
        //let _test = format!("{:?}", state);
        self.repeat_count = 0;
        self.last_state = Some(state);
        self.last_result = LastMatchResult::Failed;
        self.has_skipped_cache_fail = false;

        // failed final stat - couldn't parse anything on line
        if self.stack.is_empty() {
            self.parsing_line = false;
            self.parse_title = false;
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
        types: Types,
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
        self.stack.push(State {
            expr_index,
            types,
            first_parse: index,
            last_parse: index,
            state,
        });

        self.repeat_count = 0;
        self.last_result = LastMatchResult::New(locs);
        self.has_skipped_cache_fail = false;

        ParserResult::ContinueWith
    }

    ///this function is called if the step matches
    fn matched_func(
        &mut self,
        mut index: usize,
        return_type: ReturnType,
        closed: bool,
    ) -> ParserResult {
        let state = self.stack.pop().unwrap();
        let expr_index = state.expr_index;
        if state.state.get_type() == StateType::Stat {
            // add self
            self.stat_indexes.push(expr_index);
            // stats can change parse ablility -- reset cached fails
            self.cached_fails.clear();
        }
        self.last_state = Some(state);
        self.has_skipped_cache_fail = false;

        // matched final stat
        if self.stack.is_empty() {
            if self.parse_title {
                if let Expr::Title { data } = &self.data.exprs[expr_index] {
                    let imports = data.imports.iter().map(|e| e.0).collect::<Vec<_>>();
                    self.aliases = AliasData::new(&mut imports.iter());
                    self.data.imports = imports;
                } else {
                    unreachable!()
                };
            }
            self.parse_title = false;
            // setup next
            self.add_new_start_state(index);
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
            self.last_result = LastMatchResult::Matched(expr_index, return_type);
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
            let found_data = data.trim().len() > 0;
            if found_data {
                self.add_new_start_state(0);
                self.parsing_line = true;
            }
            found_data
        } else {
            false
        }
    }
    ///setup a noneStat on the stack
    fn add_new_start_state(&mut self, new_index: usize) {
        // if need to parse title -- put it on stack
        let state = if self.parse_title {
            get_state!(title::TitleState::new())
        } else {
            get_state!(alias::NoneState::new_stat_cont())
        };

        // push match stat on first step of line
        let expr_index = self.data.exprs.vec.len();

        self.data.exprs.vec.push(Expr::NoneStat);

        self.stack.push(State {
            expr_index,
            types: Types::Void,
            first_parse: new_index,
            last_parse: new_index,
            state,
        });

        self.data.stat_starts.push(expr_index);
        self.last_result = LastMatchResult::None;
        self.cached_fails.clear();
    }
}
