use std::borrow::Cow;

use super::*;
use alias_data::*;

///the state of the matching state machine
#[derive(Debug, PartialEq)]
enum MatchState {
    Var,
    Num,
    Color,
    StringLit,
    FunctionCallExpr,
    FunctionCallStat,
    FindAliases,
}

#[derive(Debug, PartialEq)]
pub struct WordTrigger {
    pub word_start: usize,
    pub word_end: usize,
    pub alias_trigger: Vec<u8>,
}

#[derive(Debug)]
pub struct WordTriggerArena {
    pub word_triggers: Vec<WordTrigger>,
    start_positions: HashMap<usize, usize>,
}

impl WordTriggerArena {
    pub fn add_val(&mut self, word_start: usize, word_end: usize, alias_trigger: Vec<u8>) {
        let trigger_insert = WordTrigger {
            word_start: word_start,
            word_end: word_end,
            alias_trigger: alias_trigger,
        };
        if let Some(val) = self.start_positions.get(&word_start) {
            self.word_triggers[*val] = trigger_insert;
        } else {
            self.start_positions
                .insert(word_start, self.word_triggers.len());
            self.word_triggers.push(trigger_insert);
        }
    }
    pub fn new() -> WordTriggerArena {
        WordTriggerArena {
            word_triggers: Vec::new(),
            start_positions: HashMap::new(),
        }
    }
}

///does type1 contain all types from type2
fn is_valid_type(type_from: Types, type_to: Types) -> bool {
    type_from.contains(type_to)
}

/// used for both NoneStat and NoneExpr
/// finds next command
#[derive(Debug)]
pub struct NoneState {
    ///a reference to the static data of the aliases
    data: &'static StaticAliasData,
    ///The type looked for
    types: Types,
    ///the alias looked for
    aliases: Option<Vec<&'static [u8]>>,
    ///the progress of each alias
    progress: Vec<u8>,
    ///the already parsed locs (the locations of alias characters)
    locs: Vec<Option<Vec<usize>>>,
    ///the offset into the word
    offset: usize,
    ///the number of currently matched aliases
    matched: u16,
    ///the next state of the state machine
    next_match_state: MatchState,
}

impl ParseState for NoneState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        if self.data.is_expr {
            let vec = env
                .aliases
                .expr
                .iter()
                .filter_map(|alias| is_valid_type(self.types, alias.1).then(|| alias.0))
                .collect::<Vec<_>>();

            self.aliases = Some(vec);
        }

        let aliases = self.aliases.as_ref().unwrap_or(&env.aliases.stat);
        debug_assert!(aliases.len() < u16::MAX as usize);

        // reset on new word
        self.reset(aliases.len());

        self.run_match_state(env, word, rest)
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<usize>,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if child_index.is_some() {
            // child matched successfully
            MatchResult::Matched(word.pos, false)
        } else {
            // child did not match - continue searching
            self.run_match_state(env, word, rest)
        }
    }

    fn get_name(&self) -> &'static str {
        self.data.state_name
    }

    fn get_type(&self) -> StateType {
        StateType::None
    }
}

impl NoneState {
    fn new(data: &'static StaticAliasData, types: Types) -> Self {
        Self {
            data,
            types,
            aliases: None,
            progress: Vec::new(),
            locs: Vec::new(),
            offset: 0,
            matched: 0,
            next_match_state: MatchState::FindAliases,
        }
    }
    ///reset state back to defaults for a new word
    fn reset(&mut self, length: usize) {
        self.progress = vec![0u8; length];
        self.locs = vec![Some(Vec::new()); length];
        self.offset = 0;
        self.matched = 0;
        // if expr need to check if var or num
        self.next_match_state = if self.data.is_expr {
            //Expression
            MatchState::StringLit
        } else {
            //Statement
            MatchState::FunctionCallStat
        }
    }
    pub fn new_stat() -> Self {
        Self::new(&AliasData::STAT, Types::Any)
    }
    pub fn new_stat_cont() -> Self {
        Self::new(&AliasData::STAT_CONT, Types::Any)
    }
    pub fn new_expr(types: Types) -> Self {
        Self::new(&AliasData::EXPR, types)
    }
    pub fn new_expr_cont(types: Types) -> Self {
        Self::new(&AliasData::EXPR_CONT, types)
    }
}

impl NoneState {
    ///matches based on MatchState
    ///Expr starts at Var, to check if it is a varible, then it checks if it is a number,
    ///then it tries to find aliases in the word
    ///Stat starts at the aliases directly
    fn run_match_state(
        &mut self,
        env: &mut Environment,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        let (new_state, ret) = match self.next_match_state {
            MatchState::StringLit => (
                MatchState::Var,
                MatchResult::ContinueWith(word.pos, get_state!(string_lit::LitStrState::new())),
            ),
            // is word a varible
            MatchState::Var => (
                MatchState::FunctionCallExpr,
                MatchResult::ContinueWith(word.pos, get_state!(var::VarState::new())),
            ),
            // is word a function
            MatchState::FunctionCallExpr => (
                MatchState::Num,
                MatchResult::ContinueWith(
                    word.pos,
                    get_state!(call_func::FunctionCallState::new()),
                ),
            ),
            // is word a literal number
            MatchState::Num => (
                MatchState::Color,
                MatchResult::ContinueWith(
                    word.pos,
                    get_state!(num_literal::LiteralNumState::new()),
                ),
            ),
            // is word a color
            MatchState::Color => (
                MatchState::FindAliases,
                MatchResult::ContinueWith(word.pos, get_state!(litcolor::LiteralColorState::new())),
            ),
            MatchState::FunctionCallStat => (
                MatchState::FindAliases,
                MatchResult::ContinueWith(
                    word.pos,
                    get_state!(call_func::FunctionCallState::new()),
                ),
            ),

            // else check aliases
            MatchState::FindAliases => (MatchState::FindAliases, self.match_alias(env, word, rest)),
        };
        self.next_match_state = new_state;
        ret
    }

    ///matches buildin functions based on self.data
    fn match_alias(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        let aliases = self.aliases.as_ref().unwrap_or(&env.aliases.stat);

        // run until end of word
        for offset in self.offset..word.len() {
            self.match_letters(&aliases, word, offset);

            // try match
            while self.matched != 0 {
                self.matched -= 1;
                let matched_value = self.find_best_match(word, env, &aliases, offset, rest.pos);
                return matched_value;
            }
        }

        // if default continue
        if self.data.default_continue {
            MatchResult::Continue(0)
        // else fail
        } else {
            MatchResult::Failed
        }
    }
    
    ///finds the bast match of the ones to have just matched
    ///Done by:
    ///1. Implicitly the first to match
    ///2. least length between first and last letter
    ///3. then on least total location value
    fn find_best_match(
        &mut self,
        word: &Slice,
        env: &mut Environment,
        aliases: &[StatTrigger],
        offset: usize,
        rest: usize,
    ) -> MatchResult {
        let mut min_size = usize::MAX;
        let mut min_locations = usize::MAX;
        let mut min_index = u16::MAX;
        for j in 0..aliases.len() {
            // has finished matching
            if self.progress[j] == aliases[j].len() as u8 && self.locs[j].is_some() {
                let matching_locs = self.locs[j].as_ref().unwrap();

                let size = matching_locs.last().unwrap() - matching_locs[0];
                let location_sum: usize = matching_locs.iter().sum();

                // is best match
                // match on least length between first and last letter
                // then on least total location value
                if size < min_size || (size == min_size && location_sum < min_locations) {
                    min_index = j as u16;
                    min_size = size;
                    min_locations = location_sum;
                }
            }
        }
        self.offset = offset;
        env.locs = self.locs[min_index as usize].take();
        for index in env.locs.as_mut().unwrap() {
            *index += env.global_index;
        }
        env.trigger_word_data.add_val(
            word.pos + env.global_index,
            word.pos + env.global_index + word.len(),
            aliases[min_index as usize].to_vec(),
        );
        //set up stack
        (self.data.func)(
            aliases[min_index as usize],
            rest,
            // move locs out of state without copy
        )
    }

    ///match current letter at offset to all aliases
    fn match_letters(&mut self, aliases: &[&'static [u8]], word: &Slice<'_>, offset: usize) {
        // does letter match any commands
        for i in 0..aliases.len() {
            // does letter match
            if self.progress[i] < aliases[i].len() as u8
                && word.str[offset].to_ascii_lowercase() == aliases[i][self.progress[i] as usize]
            {
                self.progress[i] += 1;
                // add locations to locations (locs)
                self.locs[i].as_mut().unwrap().push(word.pos + offset);
                if self.progress[i] == aliases[i].len() as u8 {
                    self.matched += 1;
                }
            }
        }
    }
}
