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
    WordIgnore,
    WordIgnoreStat,
    FindAliases,
}

/// used for both NoneStat and NoneExpr
/// finds next command
#[derive(Debug)]
pub struct NoneState {
    ///a reference to the static data of the aliases
    data: &'static StaticAliasData,
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
        let aliases = (self.data.aliases)(env.aliases);
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
    fn new(data: &'static StaticAliasData) -> Self {
        Self {
            data,
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
            MatchState::WordIgnoreStat
        }
    }
    pub fn new_stat() -> Self {
        Self::new(&AliasData::STAT)
    }
    pub fn new_stat_cont() -> Self {
        Self::new(&AliasData::STAT_CONT)
    }
    pub fn new_expr() -> Self {
        Self::new(&AliasData::EXPR)
    }
    pub fn new_expr_cont() -> Self {
        Self::new(&AliasData::EXPR_CONT)
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
                MatchState::WordIgnore,
                MatchResult::ContinueWith(word.pos, get_state!(string_lit::LitStrState::new()))
            ),
            MatchState::WordIgnore => (
                MatchState::Var,
                MatchResult::ContinueWith(word.pos, get_state!(ignore::IgnoreState::new()))
            ),
            // is word a varible
            MatchState::Var => (
                MatchState::FunctionCallExpr,
                MatchResult::ContinueWith(word.pos, get_state!(var::VarState::new())),
            ),
            // is word a function
            MatchState::FunctionCallExpr => (
                MatchState::Num,
                MatchResult::ContinueWith(word.pos, get_state!(call_func::FunctionCallState::new()))
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

            MatchState::WordIgnoreStat => (
                MatchState::FunctionCallStat,
                MatchResult::ContinueWith(word.pos, get_state!(ignore::IgnoreState::new()))
            ),
            MatchState::FunctionCallStat => (
                MatchState::FindAliases,
                MatchResult::ContinueWith(word.pos, get_state!(call_func::FunctionCallState::new()))
            ),
            
            // else check aliases
            MatchState::FindAliases => (MatchState::FindAliases, self.match_alias(env, word, rest)),
        };
        self.next_match_state = new_state;
        ret
    }

    ///matches buildin functions based on self.data
    fn match_alias(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        let aliases = (self.data.aliases)(env.aliases);

        // run until end of word
        for offset in self.offset..word.len() {
            self.match_letters(&aliases, word, offset);

            // try match
            while self.matched != 0 {
                self.matched -= 1;
                return self.find_best_match(env, &aliases, offset, rest.pos);
            }
        }

        // if default continue
        if self.data.default_continue {
            MatchResult::Continue
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
        env: &mut Environment,
        aliases: &AliasNames,
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
        //set up stack
        (self.data.func)(
            aliases[min_index as usize],
            rest,
            // move locs out of state without copy
        )
    }

    ///match current letter at offset to all aliases
    fn match_letters(&mut self, aliases: &AliasNames, word: &Slice<'_>, offset: usize) {
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
