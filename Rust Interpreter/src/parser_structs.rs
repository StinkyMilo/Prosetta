use std::{
    collections::HashSet,
    fmt::{self, Debug},
};

use super::{alias_data::AliasData, Expr};

#[path = "testing/parsing_tests_word_funcs.rs"]
mod parsing_tests_word_funcs;

pub type VarSet = HashSet<Vec<u8>>;
pub type StepFunction =
    fn(env: &mut Enviroment, result: LastMatchResult, word: &Slice, rest: &Slice) -> MatchResult;

// (expr_index, string_index, state)
pub type State = (usize, usize, Box<dyn ParseState>);

// pub trait ParserSource: BufRead + Debug {}
// impl<T: BufRead + Debug> ParserSource for T {}

macro_rules! get_state {
    ($state:expr) => {
        Box::new($state) as Box<dyn ParseState>
    };
}
pub(crate) use get_state;

#[derive(Default, Debug)]
pub struct ParserFlags {
    pub not: bool,
}

#[derive(PartialEq, Debug)]
pub struct BuiltinMatchState {
    progress: Vec<u8>,
    locs: Vec<Option<Vec<usize>>>,
    offset: usize,
    matched: u16,
    is_expr: bool,
}

pub trait ParseState: Debug {
    /// call first time to setup the state
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult;

    /// call rest of times after match
    fn step_match(
        &mut self,
        env: &mut Enviroment,
        child: Option<usize>,
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

#[derive(PartialEq, Debug)]
pub enum LastMatchResult {
    None,
    New(Option<Vec<usize>>),
    Matched(usize),
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

impl ParserResult {
    pub fn is_end(&self) -> bool {
        matches!(
            self,
            ParserResult::MatchedLine | ParserResult::FailedLine | ParserResult::NoInput
        )
    }
}

pub struct Enviroment<'a> {
    pub vars: &'a VarSet,
    pub expr: &'a mut Expr,
    pub locs: Option<Vec<usize>>,
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
    pub fn len(&self) -> usize {
        self.str.len()
    }
    pub fn end(&self) -> usize {
        self.pos + self.str.len()
    }
    pub fn offset(&self, offset: usize) -> Slice {
        Slice {
            str: &self.str[offset..],
            pos: self.pos + offset,
        }
    }
    pub fn extend(&self) -> Slice {
        Slice {
            str: &self.str[self.pos..],
            pos: self.pos,
        }
    }
}

fn is_valid_word_char(char: u8) -> bool {
    char.is_ascii_alphanumeric() || char == b'-'
}

const END_CHARS: &[u8] = b".?!,";
fn is_valid_close_char(char: u8) -> bool {
    END_CHARS.contains(&char)
}

pub fn get_next_word<'a>(slice: &Slice<'a>, mut start: usize) -> (Slice<'a>, Slice<'a>) {
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

pub fn get_next_slice<'a>(slice: &Slice<'a>, mut start: usize) -> (Slice<'a>, Slice<'a>) {
    // find start of word
    start = start.min(slice.len());
    while start < slice.len()
        && !is_valid_word_char(slice.str[start])
        && !is_valid_close_char(slice.str[start])
    {
        start += 1;
    }

    // find end of word
    let mut end = start;

    //is slice = "."
    if end < slice.len() && is_valid_close_char(slice.str[end]) {
        end += 1;
    } else {
        while end < slice.len() && is_valid_word_char(slice.str[end]) {
            end += 1;
        }
    }
    // }

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
pub fn find_word_end<'a>(slice: &'a Slice<'a>, start: usize) -> Slice<'a> {
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

// returns the rest after finding the end of an sentence
pub fn find_close<'a>(slice: &'a Slice<'a>, start: usize) -> Option<Slice<'_>> {
    // find end char
    let mut end = start;
    while end < slice.len() && !is_valid_close_char(slice.str[end]) {
        end += 1;
    }
    let test = end < slice.len();
    //end += 1;
    // find end of period
    test.then(|| Slice {
        str: &slice.str[end..],
        pos: slice.pos + end,
    })
}

pub fn is_close(slice: &Slice) -> bool {
    slice.len() > 0 && is_valid_close_char(slice.str[0])
}


