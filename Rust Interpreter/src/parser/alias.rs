use super::*;
use alias_data::*;

///the state of the matching state machine
#[derive(Debug, PartialEq, Clone, Copy)]
enum MatchState {
    Var,
    Num,
    Color,
    StringLit,
    FunctionCallExpr,
    FunctionCallStat,
    FindAliases,
    Comment,
}

#[derive(Debug, PartialEq)]
pub enum WordTriggerType{
    Alias(Vec<u8>),
    Length(usize, bool),
    Variable(Vec<u8>),
    Import(Vec<u8>)
}

#[derive(Debug, PartialEq)]
pub struct WordTrigger {
    pub word_start: usize,
    pub word_end: usize,
    pub trigger_data: WordTriggerType
}

#[derive(Debug)]
pub struct WordTriggerArena {
    pub word_triggers: Vec<WordTrigger>,
    start_positions: HashMap<usize, usize>,
}

impl WordTriggerArena {
    pub fn add_val(&mut self, word_start: usize, word_end: usize, trigger_data: WordTriggerType) {
        let trigger_insert = WordTrigger {
            word_start: word_start,
            word_end: word_end,
            trigger_data: trigger_data,
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
fn is_valid_type(type_wants: Types, type_has: Types) -> bool {
    type_wants.contains(type_has)
}

#[derive(Debug)]
struct AliasFinder {
    ///the progress of each alias
    progress: Vec<u8>,
    ///the already parsed locs (the locations of alias characters)
    locs: Vec<Option<Vec<usize>>>,
    ///the offset into the word
    offset: usize,
    ///the number of currently matched aliases
    matched: u16,
}

/// used for both NoneStat and NoneExpr
/// finds next command
#[derive(Debug)]
pub struct NoneState {
    ///a reference to the static data of the aliases
    data: &'static StaticAliasData,
    ///the alias looked for
    aliases: Option<Vec<&'static [u8]>>,
    ///the next state of the state machine
    next_match_state: MatchState,
    //alias finders
    alias_finder: AliasFinder,
}

impl ParseState for NoneState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        let length = if self.data.is_expr {
            let vec = env
                .aliases
                .expr
                .iter()
                .filter_map(|alias| is_valid_type(env.types, alias.1).then(|| alias.0))
                .collect::<Vec<_>>();
            let length = vec.len();
            self.aliases = Some(vec);
            length
        } else {
            env.aliases.stat.len()
        };

        debug_assert!(length < u16::MAX as usize);
        // reset on new word
        self.reset(length);

        self.run_match_state(env, word, rest)
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<(usize, ReturnType)>,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if let Some((_, return_type)) = child_index {
            // child matched successfully
            MatchResult::Matched(word.pos, return_type, false)
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

impl AliasFinder {
    fn new(length: usize) -> Self {
        Self {
            progress: vec![0u8; length],
            locs: vec![Some(Vec::new()); length],
            offset: 0,
            matched: 0,
        }
    }
}

impl NoneState {
    fn new(data: &'static StaticAliasData) -> Self {
        Self {
            data,
            aliases: None,
            next_match_state: MatchState::FindAliases,
            alias_finder: AliasFinder {
                progress: Vec::new(),
                locs: Vec::new(),
                offset: 0,
                matched: 0,
            },
        }
    }
    ///reset state back to defaults for a new word
    fn reset(&mut self, length: usize) {
        self.alias_finder = AliasFinder::new(length);
        // if expr need to check if var or num
        self.next_match_state = if self.data.is_expr {
            //Expression
            MatchState::StringLit
        } else {
            //Statement
            MatchState::Comment
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
    fn get_new_state(
        state: &MatchState,
        types: Types,
    ) -> (MatchState, Option<Box<dyn ParseState>>) {
        match state {
            MatchState::StringLit => (
                MatchState::Var,
                is_valid_type(types, Types::String)
                    .then(|| get_state!(string_lit::LitStrState::new())),
            ),
            // is word a varible
            MatchState::Var => (
                MatchState::FunctionCallExpr,
                // var is any
                Some(get_state!(var::VarState::new())),
            ),
            // is word a function
            MatchState::FunctionCallExpr => (
                MatchState::Num,
                // func is any
                Some(get_state!(call_func::FunctionCallState::new())),
            ),
            // is word a literal number
            MatchState::Num => (
                MatchState::Color,
                is_valid_type(types, Types::Number)
                    .then(|| get_state!(num_literal::LiteralNumState::new())),
            ),
            // is word a color
            MatchState::Color => (
                MatchState::FindAliases,
                is_valid_type(types, Types::Color)
                    .then(|| get_state!(litcolor::LiteralColorState::new())),
            ),
            MatchState::Comment => (
                MatchState::FunctionCallStat,
                Some(get_state!(comment::CommentState::new())),
            ),
            MatchState::FunctionCallStat => (
                MatchState::FindAliases,
                Some(get_state!(call_func::FunctionCallState::new())),
            ),
            MatchState::FindAliases => unreachable!(),
        }
    }

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
        let mut ret = None;
        while ret.is_none() {
            (self.next_match_state, ret) = match &self.next_match_state {
                // else check aliases
                MatchState::FindAliases => (
                    MatchState::FindAliases,
                    Some(self.match_alias(env, word, rest)),
                ),
                other => {
                    let (match_state, state) = Self::get_new_state(other, env.types);
                    (
                        match_state,
                        state.map(|state| MatchResult::ContinueWith(word.pos, env.types, state)),
                    )
                }
            }
        }
        ret.unwrap()
    }

    ///matches buildin functions based on self.data
    fn match_alias(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        let aliases = self.aliases.as_ref().unwrap_or(&env.aliases.stat);
        if let Some((alias, locs)) = self.alias_finder.run(aliases, word) {
            env.locs = locs;
            for index in env.locs.as_mut().unwrap() {
                *index += env.global_index;
            }

            env.trigger_word_data.add_val(
                word.pos + env.global_index,
                word.pos + env.global_index + word.len(),
                WordTriggerType::Alias(alias.to_vec()),
            );
            //set up stack
            let state = (self.data.func)(alias);
            MatchResult::ContinueWith(rest.pos, env.types, state)
        }
        // if default continue
        else if self.data.default_continue {
            MatchResult::Continue(0)
        // else fail
        } else {
            MatchResult::Failed
        }
    }
}

impl AliasFinder {
    fn run(
        &mut self,
        aliases: &[StatTrigger],
        word: &Slice,
    ) -> Option<(StatTrigger, Option<Vec<usize>>)> {
        // run until end of word
        while self.offset < word.len() {
            self.match_letters(&aliases, word);

            // try match
            while self.matched != 0 {
                self.matched -= 1;
                let alias_index = self.find_best_match(aliases);
                let locs = self.locs[alias_index as usize].take();
                return Some((aliases[alias_index as usize], locs));
            }
            self.offset += 1;
        }
        return None;
    }

    ///finds the bast match of the ones to have just matched
    ///Done by:
    ///1. Implicitly the first to match
    ///2. least length between first and last letter
    ///3. then on least total location value
    fn find_best_match(&mut self, aliases: &[StatTrigger]) -> u16 {
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
        return min_index;
    }

    ///match current letter at offset to all aliases
    fn match_letters(&mut self, aliases: &[&'static [u8]], word: &Slice<'_>) {
        // does letter match any commands
        for i in 0..aliases.len() {
            // does letter match
            if self.progress[i] < aliases[i].len() as u8
                && word.str[self.offset].to_ascii_lowercase()
                    == aliases[i][self.progress[i] as usize]
            {
                self.progress[i] += 1;
                // add locations to locations (locs)
                self.locs[i].as_mut().unwrap().push(word.pos + self.offset);
                if self.progress[i] == aliases[i].len() as u8 {
                    self.matched += 1;
                }
            }
        }
    }
}
