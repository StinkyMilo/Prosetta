use bstr::ByteSlice;
use quickscope::ScopeMap;
use std::{
    collections::HashSet,
    fmt::{self, Debug},
    usize,
};

use super::{alias::WordTriggerArena, alias_data::AliasData, Expr};

pub fn try_get_best_val<'a>(
    name: &[u8],
    iter: &mut dyn Iterator<Item = &'a [u8]>,
    pred: &dyn Fn(&[u8]) -> bool,
) -> Option<(u8, &'a [u8], usize)> {
    let mut max_var_length = 0u8;
    let mut var_data: Option<(u8, &[u8], usize)> = None;
    for (index, str) in iter.enumerate() {
        let is_longer = str.len() as u8 >= max_var_length;
        // if var could be in word
        if is_longer && name.len() >= str.len() && (pred)(str) {
            // if found
            if let Some(str_index) = name.find(str) {
                let is_better = var_data.as_ref().map_or(true, |&(old_index, _, _)| {
                    is_longer || (str_index as u8) < old_index
                });

                if is_better {
                    max_var_length = str.len() as u8;
                    var_data = Some((str_index as u8, str, index));
                }
            }
        }
    }
    var_data
}
/// remove skip indexes outside of var str and fix indexes for ones inside, returns var start in term of word start
fn convert_skip_indexes(skip_indexes: &mut Vec<u8>, var_start: u8, var_len: u8) -> u8 {
    let mut start_index = 0u8;
    while start_index < skip_indexes.len() as u8 {
        if skip_indexes[start_index as usize] <= var_start + start_index {
            start_index += 1;
        } else {
            break;
        }
    }

    let start = var_start + start_index;
    let var_end = start + var_len;
    skip_indexes.drain(..start_index as usize);

    let end_index = skip_indexes
        .iter()
        .position(|&v| v > var_end)
        .unwrap_or(skip_indexes.len());
    skip_indexes.drain(end_index..);

    for val in skip_indexes {
        *val -= start;
    }
    start
}

fn try_get_from_iter<'a>(
    word: &Slice,
    iter: &mut dyn Iterator<Item = &'a [u8]>,
    global_index: usize,
    pred: &dyn Fn(&[u8]) -> bool,
) -> Option<SubStrData> {
    if word.len() > 255 {
        return None;
    }
    // remove ' and make lowercase
    let (name, mut skip_indexes) = get_var_name_and_skips(word.str);
    let var_data = try_get_best_val(&name, iter, pred);

    if let Some((var_start, name, _)) = var_data {
        let start = convert_skip_indexes(&mut skip_indexes, var_start, name.len() as u8);

        Some(SubStrData {
            start: global_index + word.pos + start as usize,
            name: name.to_vec(),
            skip_indexes,
        })
    } else {
        None
    }
}
enum Symbol {
    ///symbol is a varible
    Var,
    ///symbol is a function with a number of arguments
    Func(u8),
}

pub struct SymbolSet {
    set: ScopeMap<Vec<u8>, Symbol>,
}
impl SymbolSet {
    pub fn new() -> Self {
        Self {
            set: ScopeMap::new(),
        }
    }
    pub fn insert_var(&mut self, mut name: Vec<u8>) {
        name.make_ascii_lowercase();
        self.set.define(name, Symbol::Var);
    }
    pub fn insert_func(&mut self, mut name: Vec<u8>, args: u8) {
        name.make_ascii_lowercase();
        self.set.define(name, Symbol::Func(args));
    }
    pub fn add_layer(&mut self) {
        self.set.push_layer();
    }
    pub fn remove_layer(&mut self) {
        self.set.pop_layer();
    }
    pub fn contains(&self, name: &[u8]) -> bool {
        let lower = name.to_ascii_lowercase();
        self.set.contains_key(&lower)
    }
    pub fn get_func_arg_count(&self, name: &Vec<u8>) -> Option<u8> {
        let lower = name.to_ascii_lowercase();
        if let Some(Symbol::Func(count)) = self.set.get(&lower) {
            Some(*count)
        } else {
            None
        }
    }
    ///returns (index in word, varible name)
    ///
    pub fn try_get_var(&self, word: &Slice, global_index: usize) -> Option<SubStrData> {
        try_get_from_iter(
            word,
            &mut self.set.keys().map(|e| e.as_slice()),
            global_index,
            &|name| matches!(self.set.get(name), Some(Symbol::Var)),
        )
    }

    pub fn try_get_func(&self, word: &Slice, global_index: usize) -> Option<SubStrData> {
        try_get_from_iter(
            word,
            &mut self.set.keys().map(|e| e.as_slice()),
            global_index,
            &|name| matches!(self.set.get(name), Some(Symbol::Func(..))),
        )
    }
}
impl Debug for SymbolSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SymbolSet").finish()
    }
}

pub struct IgnoreSet {
    set: HashSet<Vec<u8>>,
}

impl IgnoreSet {
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }
    pub fn insert(&mut self, name: Vec<u8>) {
        let lower = name.to_ascii_lowercase();
        self.set.insert(lower);
    }
    pub fn contains(&self, name: &Vec<u8>) -> bool {
        let lower = name.to_ascii_lowercase();
        self.set.contains(&lower)
    }
    ///returns (index in word, length)
    pub fn try_get_val(&self, word: &Slice, global_index: usize) -> Option<SubStrData> {
        if word.len() > 255 {
            return None;
        }
        // remove ' and make lowercase
        let (name, mut skip_indexes) = get_var_name_and_skips(word.str);
        let var_data =
            try_get_best_val(&name, &mut self.set.iter().map(|e| e.as_slice()), &|_| true);

        if let Some((var_start, name, _)) = var_data {
            let start = convert_skip_indexes(&mut skip_indexes, var_start, name.len() as u8);

            Some(SubStrData {
                start: global_index + word.pos + start as usize,
                name: name.to_vec(),
                skip_indexes,
            })
        } else {
            None
        }
    }
}

impl Debug for IgnoreSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IgnoreSet").finish()
    }
}

// pub struct FuncSet {
//     /// set with <name, arg_count>
//     set: ScopeMap<Vec<u8>, usize>,
// }

// impl FuncSet {
//     pub fn new() -> Self {
//         Self {
//             set: ScopeMap::new(),
//         }
//     }
//     pub fn insert(&mut self, name: Vec<u8>, arg_count: usize) {
//         self.set.define(name, arg_count);
//     }
//     pub fn add_layer(&mut self) {
//         self.set.push_layer();
//     }
//     pub fn remove_layer(&mut self) {
//         self.set.pop_layer();
//     }
//     pub fn contains(&self, name: Vec<u8>) -> bool {
//         self.set.contains_key(&name)
//     }
//     pub fn try_get_func(&self, word: &Slice, global_index: usize) -> Option<SubStrData> {
//         if word.len() > 255 {
//             return None;
//         }
//         // remove ' and make lowercase
//         let (name, mut skip_indexes) = get_var_name_and_skips(word.str);
//         let var_data = try_get_val(&name, &mut self.set.keys());

//         if let Some((var_start, name)) = var_data {
//             let start = convert_skip_indexes(&mut skip_indexes, var_start, name.len() as u8);

//             Some(SubStrData {
//                 start: global_index + word.pos + start as usize,
//                 name: name.to_vec(),
//                 skip_indexes,
//             })
//         } else {
//             None
//         }
//     }

//     pub fn get_arg_count(&self, name: &[u8]) -> Option<&usize> {
//         self.set.get(name)
//     }
// pub fn inc_arg_count(&mut self, name: &[u8]) {
//     if let Some(val) = self.set.get(name) {
//         //Increment in above scope.
//         self.set.define(name.to_vec(), val + 1);
//     }
// }
// }

// impl Debug for FuncSet {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("FuncSet").finish()
//     }
// }
#[derive(PartialEq, Debug)]
pub struct SubStrData {
    pub name: Vec<u8>,
    pub start: usize,
    pub skip_indexes: Vec<u8>,
}

impl SubStrData {
    pub fn new() -> Self {
        Self {
            start: usize::MAX,
            name: Vec::new(),
            skip_indexes: Vec::new(),
        }
    }
}

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
pub(crate) use get_state;

// macro_rules! only_debug {
//     ($expr:expr) => {
//         if cfg!(debug_assertions) {
//             format!("{:?}", $expr)
//         } else {
//             Default::default()
//         }
//     };
// }
// pub(crate) use only_debug;

/// add or remove commands based on flags
#[derive(Default, Debug)]
pub struct ParserFlags {
    pub title: bool,
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
    /// returned to give the same state with the offset (usually 0)
    Continue(usize),
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
    pub symbols: &'a mut SymbolSet,
    ///The set of current ignored values
    pub nots: &'a mut IgnoreSet,
    ///The list of expressions
    pub expr: &'a mut Expr,
    ///the index of this expr
    pub expr_index: usize,
    ///the exprs before this
    pub parents: &'a [State],
    ///the exprs before this
    pub before: &'a mut [Expr],
    ///the exprs after this
    pub after: &'a mut [Expr],
    ///The last matched expr if exists
    pub last_stat_index: Option<usize>,
    ///The current locs (locations of the alias characters)
    pub locs: Option<Vec<usize>>,
    /// the global index (with multiple input buffers)
    pub global_index: usize,
    /// reference to static AliasData
    pub aliases: &'a AliasData,
    /// full text of the poem
    pub full_text: &'a [u8],
    /// The global start and end of alias data
    pub trigger_word_data: &'a mut WordTriggerArena
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
const OTHER_CHARS: &[u8] = b"-+^/'";
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
const NON_CLOSE_CHARS: &[u8] = b"\"&";
///shoudl the char be made into a 1 len slice
pub fn is_non_close_but_still_single(char: u8) -> bool {
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
pub struct CloseData {
    pub close_count: u8,
    pub close_length: u8,
    pub only_forced: bool,
}
/// gets the number of times the characters at line[index] should be repeated and the offset after
/// returns (repeat_count,offset)
pub fn get_close_data(line: &[u8]) -> CloseData {
    if line.len() >= 3 && line[..3] == b"..."[..] {
        CloseData {
            close_count: 10,
            close_length: 3,
            only_forced: false,
        }
    } else if line.len() >= 3 && line[..3] == b"---"[..] {
        CloseData {
            close_count: 3,
            close_length: 3,
            only_forced: false,
        }
    } else if line.len() >= 2 && line[..2] == b"--"[..] {
        CloseData {
            close_count: 2,
            close_length: 2,
            only_forced: false,
        }
    } else if line.len() >= 1 {
        match line[0] {
            b'.' | b':' => CloseData {
                close_count: 1,
                close_length: 1,
                only_forced: false,
            },
            b',' | b';' => CloseData {
                close_count: 1,
                close_length: 1,
                only_forced: true,
            },
            b'?' | b'!' => CloseData {
                close_count: 2,
                close_length: 1,
                only_forced: false,
            },
            _ => CloseData {
                close_count: 0,
                close_length: 0,
                only_forced: false,
            },
        }
    } else {
        CloseData {
            close_count: 0,
            close_length: 0,
            only_forced: false,
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

pub fn get_var_name_and_skips(word: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut name = Vec::new();
    let mut skips = Vec::new();
    for j in 0..word.len() {
        if word[j] == b'\'' {
            skips.push(j as u8);
        } else {
            name.push(word[j].to_ascii_lowercase());
        }
    }
    (name, skips)
}

pub fn try_get_symbol_word(word: &Slice, global_index: usize) -> Option<SubStrData> {
    if word.len() >= 3
        && word.len() <= 255
        && !is_close(word)
        && !is_non_close_but_still_single(word.str[0])
    {
        let (name, skip_indexes) = get_var_name_and_skips(word.str);

        // vars cant be empty
        if name.is_empty() {
            None
        } else {
            Some(SubStrData {
                start: global_index + word.pos,
                name,
                skip_indexes,
            })
        }
    } else {
        None
    }
}
///get a slice that starts at the next \n
pub fn find_newline<'a>(slice: &'a Slice<'a>, mut start: usize) -> Option<Slice<'_>> {
    while start < slice.len() {
        let char = slice.str[start];
        if char == b'\n' {
            break;
        } else {
            start += 1;
        }
    }

    if start < slice.len() {
        let end = start;
        Some(Slice {
            str: &slice.str[start..end],
            pos: slice.pos + start,
        })
    } else {
        None
    }
}
