use std::{
    fmt::{self, Debug},
    usize,
};

use super::{alias_data::AliasData, Expr};

#[path = "testing/parsing_tests_word_funcs.rs"]
mod parsing_tests_word_funcs;
// quickscope

pub struct VarSet {
    set: ScopeSet<Vec<u8>>,
}
impl VarSet {
    pub fn new() -> Self {
        Self {
            set: ScopeSet::new(),
        }
    }
    pub fn insert(&mut self, name: Vec<u8>) {
        let lower = name.to_ascii_lowercase();
        self.set.define(lower);
    }
    pub fn add_layer(&mut self) {
        self.set.push_layer();
    }
    pub fn remove_layer(&mut self) {
        self.set.pop_layer();
    }
    pub fn contains(&self, name: &Vec<u8>) -> bool {
        let lower = name.to_ascii_lowercase();
        self.set.contains(&lower)
    }
    ///returns (index in word, varible name)
    pub fn try_get_var(&self, word: &[u8]) -> Option<(usize, Vec<u8>)> {
        let lower = word.to_ascii_lowercase();
        let mut max_var_length = 0;
        let mut var = None;
        for str in self.set.iter() {
            let is_longer = str.len() >= max_var_length;
            // if var could be in word
            if is_longer && lower.len() >= str.len() {
                // if found
                if let Some(index) = word.find(str) {
                    let is_better = var
                        .as_ref()
                        .map_or(true, |(old_index, _)| is_longer || index < *old_index);

                    if is_better {
                        max_var_length = str.len();
                        var = Some((index, str.clone()));
                    }
                }
            }
        }
        var
    }
}
impl Debug for VarSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VarSet").finish()
    }
}

pub struct FuncSet {
    set: ScopeMap<Vec<u8>, usize>
}

impl Debug for FuncSet{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FuncSet").finish()
    }
}

impl FuncSet{
    pub fn new () -> Self {
        Self {
            set: ScopeMap::new()
        }
    }
    pub fn insert(&mut self, name: Vec<u8>, arg_count: usize) {
        self.set.define(name, arg_count);
    }
    pub fn add_layer(&mut self){
        self.set.push_layer();
    }
    pub fn remove_layer(&mut self){
        self.set.pop_layer();
    }
    pub fn contains(&self, name: Vec<u8>) -> bool {
        self.set.contains_key(&name)
    }
    pub fn get_arg_count(&self, name: Vec<u8>) -> Option<&usize> {
        self.set.get(&name)
    }
    pub fn inc_arg_count(&mut self, name: Vec<u8>) {
        if let Some(val) = self.set.get(&name) {
            //Increment in above scope.
            self.set.define(name.to_vec(), val+1);
        }
    }
}
// pub type StepFunction =
//     fn(env: &mut Environment, result: LastMatchResult, word: &Slice, rest: &Slice) -> MatchResult;

// (expr_index, string_index, state)

/// a state on the stack
/// State.0 is the index of the expr in the list
/// State.1 is the last string parse location
/// State.2 is the state itself
//pub type State = (usize, usize, Box<dyn ParseState>);
#[derive(Debug)]

/// a state on the stack
pub struct State {
    pub expr_index: usize,
    pub first_parse: usize,
    pub last_parse: usize,
    pub state: Box<dyn ParseState>,
}

/// a macro to change the a ParseState to a generic box
macro_rules! get_state {
    ($state:expr) => {
        Box::new($state) as Box<dyn ParseState>
    };
}
use bstr::ByteSlice;
pub(crate) use get_state;
use quickscope::{ScopeMap, ScopeSet};

/// add or remove commands based on flags
#[derive(Default, Debug)]
pub struct ParserFlags {
    pub not: bool,
}

#[derive(PartialEq, Eq, Debug)]
pub enum StateType {
    /// for NoneExpr and NoneStat
    /// is replaceble
    None,
    Expr,
    Stat,
}

/// A state (which goes onto the parser stack)
pub trait ParseState: Debug {
    /// called first time to setup the state and after the state continues
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult;

    /// called after match or fail
    fn step_match(
        &mut self,
        env: &mut Environment,
        child: Option<usize>,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult;

    /// gets the name of the state
    fn get_name(&self) -> &'static str;

    fn get_type(&self) -> StateType;
}

///a struct for closing character with an index and a length
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct End {
    pub index: usize,
    pub count: u8,
}

impl End {
    /// make new end from index and close length
    pub fn new(index: usize, count: u8) -> Self {
        Self { index, count }
    }

    pub fn from_slice(slice: &Slice, global_index: usize) -> Self {
        End::new(slice.pos + global_index, slice.str.len() as u8)
    }

    pub fn none() -> Self {
        End::new(usize::MAX, 0)
    }
}

/// the result of a step or stepmatch function
#[derive(Debug)]
pub enum MatchResult {
    /// returned to go to the parent state with the index to now parse from and whether the state closed on it
    Matched(usize, bool),
    /// returned to add a child onto the stack with an index and the state to continue with
    ContinueWith(usize, Box<dyn ParseState>),
    /// returned to give the same state the next word
    Continue,
    /// returned to go to the parent state with a failure
    Failed,
}

///the result of the last match
///None means that the parser just started
///New means that continuewith was returned with the locs if they exist
///The rest are the same as MatchResult
#[derive(PartialEq, Debug)]
pub enum LastMatchResult {
    None,
    New(Option<Vec<usize>>),
    Matched(usize),
    Failed,
    Continue,
}

///The state that is returned each step
///NoInput means that the parser ran out of input text
///Start means that the parser just started (it is never returned)
///MatchedLine means that the parser just matched a statement
///FailedLine means that the parser just reached the end of a buffer without matching
///The rest are the same as MatchResult and returned accordingly   
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
    /// this fail was cached earlier
    CachedFail,
}

///is the state able to be closed
///either it can't, it can, or it must.
#[derive(Debug)]
pub enum CloseType {
    Unable,
    Able,
    Force,
}

impl ParserResult {
    /// is the parser result an end (is it MatchedLine, FailedLine, or NoInput)
    pub fn is_end(&self) -> bool {
        matches!(
            self,
            ParserResult::MatchedLine | ParserResult::FailedLine | ParserResult::NoInput
        )
    }
}

///the parser enviorment
pub struct Environment<'a> {
    ///The set of current varibles
    pub vars: &'a mut VarSet,
    //The set of current functions
    pub funcs: &'a mut FuncSet,
    ///The list of expressions
    pub expr: &'a mut Expr,
    ///the index of this expr
    pub expr_index: usize,
    ///the exprs before this
    pub parents: &'a mut [Expr],
    ///the exprs after this
    pub children: &'a mut [Expr],
    ///The last matched expr if exists
    pub last_stat_index: Option<usize>,
    ///The current locs (locations of the alias characters)
    pub locs: Option<Vec<usize>>,
    /// the global index (with multiple input buffers)
    pub global_index: usize,
    /// reference to static AliasData
    pub aliases: &'a AliasData,
}

impl<'a> Environment<'a> { 
    pub fn add_var_layer(&mut self){
        self.vars.add_layer();
        self.funcs.add_layer();
    }
    pub fn remove_var_layer(&mut self){
        self.vars.remove_layer();
        self.funcs.remove_layer();
    }
}

///a slice of the input text
#[derive(PartialEq)]
pub struct Slice<'a> {
    ///the string itself
    pub str: &'a [u8],
    ///the position relative to the buffer
    pub pos: usize,
}

///Slice Debug impl
impl<'a> fmt::Debug for Slice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Slice")
            .field("str", &String::from_utf8_lossy(&self.str))
            .field("pos", &self.pos)
            .finish()
    }
}

impl<'a> Slice<'a> {
    ///the length of the slice
    ///the same as .str.len()
    pub fn len(&self) -> usize {
        self.str.len()
    }
    ///the end of the slice relative to the buffer
    pub fn end(&self) -> usize {
        self.pos + self.str.len()
    }
    ///returns a new slice that is shortened by offset
    pub fn offset(&self, offset: usize) -> Slice {
        Slice {
            str: &self.str[offset..],
            pos: self.pos + offset,
        }
    }
}

///the chars that are counted as being part of words
const OTHER_CHARS: &[u8] = b"-+^/";
///can the char be part of a word
fn is_valid_word_char(char: u8) -> bool {
    char.is_ascii_alphanumeric() || OTHER_CHARS.contains(&char)
}

///chars that close functions
const END_CHARS: &[u8] = b".?!,:";
///can the char close a command
fn is_valid_close_char(char: u8) -> bool {
    END_CHARS.contains(&char)
}

///the chars that are returned single but are not closes
const NON_CLOSE_CHARS: &[u8] = b"\"\'";
///shoudl the char be made into a 1 len slice
fn is_non_close_but_still_single(char: u8) -> bool {
    NON_CLOSE_CHARS.contains(&char)
}

/// does slice consist of a closing character
pub fn is_close(slice: &Slice) -> bool {
    // does str close something
    get_close_data(slice.str).close_length != 0
}

/// For when a close is forced rather than able.
pub fn is_mandatory_close(slice: &Slice) -> bool {
    let cd = get_close_data(slice.str);
    cd.close_length != 0 && !cd.only_forced
}

///get the next valid word and the rest of the string as decided by is_valid_word_char()
///returns (word,rest)
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
pub struct CloseData{
    pub close_count: u8,
    pub close_length: u8,
    pub only_forced: bool
}
/// gets the number of times the characters at line[index] should be repeated and the offset after
/// returns (repeat_count,offset)
pub fn get_close_data(line: &[u8]) -> CloseData {
    if line.len() >= 3 && line[..3] == b"..."[..] {
        CloseData{
            close_count: 10,
            close_length: 3,
            only_forced: false
        }
    } else if line.len() >= 3 && line[..3] == b"---"[..] {
        CloseData{
            close_count: 3,
            close_length: 3,
            only_forced: false
        }
    } else if line.len() >= 2 && line[..2] == b"--"[..] {
        CloseData{
            close_count: 2,
            close_length: 2,
            only_forced: false
        }
    } else if line.len() >= 1 {
        match line[0] {
            b'.' | b':' => CloseData {
                close_count: 1,
                close_length: 1,
                only_forced: false
            },
            b',' | b';' => CloseData {
                close_count: 1,
                close_length: 1,
                only_forced: true
            },
            b'?' | b'!' => CloseData {
                close_count: 2,
                close_length: 1,
                only_forced: false
            },
            _ => CloseData {
                close_count: 0,
                close_length: 0,
                only_forced: false
            },
        }
    } else {
        CloseData {
            close_count: 0,
            close_length: 0,
            only_forced: false
        }
    }
}

///gets the next slice. a slice consists of either a word or a closing character
pub fn get_next_slice<'a>(slice: &Slice<'a>, mut start: usize) -> (Slice<'a>, Slice<'a>) {
    // find start of word
    start = start.min(slice.len());
    while start < slice.len()
        && !is_valid_word_char(slice.str[start])
        && !is_valid_close_char(slice.str[start])
        && !is_non_close_but_still_single(slice.str[start])
    {
        start += 1;
    }

    // find end of word
    let mut end = start;

    //is slice a closing character aka "."
    // if end < slice.len()
    //     && (is_valid_close_char(slice.str[end]) || is_non_close_but_still_single(slice.str[start]))
    // {
    //     // is "..."
    //     if end + 3 <= slice.len() && &slice.str[end..end + 3] == &b"..."[..] {
    //         end += 3;
    //     // not "..."
    //     } else {
    //         end += 1;
    //     }
    let close_data = get_close_data(&slice.str[start..]);
    if close_data.close_length != 0 {
        end += close_data.close_length as usize;
    } else if end < slice.len() && is_non_close_but_still_single(slice.str[start]) {
        end += 1;
    } else {
        while end < slice.len() && is_valid_word_char(slice.str[end]) {
            end += 1;
        }
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

/// returns the rest after the end of the word
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

/// returns (close, rest) after finding close
pub fn find_close_slice<'a>(slice: &'a Slice<'a>, mut start: usize) -> Option<(Slice, Slice)> {
    // find end char
    let mut close_len = 0;
    while start < slice.len() {
        close_len = get_close_data(&slice.str[start..]).close_length;
        if close_len == 0 {
            start += 1;
        } else {
            break;
        }
    }
    if start < slice.len() {
        // find end of period
        let end = start + close_len as usize;
        Some((
            Slice {
                str: &slice.str[start..end],
                pos: slice.pos + start,
            },
            Slice {
                str: &slice.str[end..],
                pos: slice.pos + end,
            },
        ))
    } else {
        None
    }
}

/// returns the rest after finding the next closing character
pub fn find_close<'a>(slice: &'a Slice<'a>, start: usize) -> Option<Slice<'_>> {
    find_close_slice(slice, start).map(|s| s.1)
}
