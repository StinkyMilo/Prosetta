use super::*;
use alias_data::*;

/// used for both NoneStat and NoneExpr
/// finds next buildin function
#[derive(Debug)]
pub struct NoneState<'a> {
    data: &'a BuiltinData,
    progress: Vec<u8>,
    locs: Vec<Option<Vec<usize>>>,
    offset: usize,
    matched: u16,
}

impl<'a> ParseState for NoneState<'a> {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        debug_assert!(self.data.names.len() < u16::MAX as usize);

        // reset on new word
        self.reset();

        // if expr  - check if varible name
        if self.data.is_expr {
            let mut var_state = var::VarState::new();
            // check if word is varible
            // continue if it is
            if var_state.check(env, word) {
                return MatchResult::ContinueWith(word.pos, Box::new(var_state));
            }
            // check if word is literal number
            // continue if it is
            let mut num_state = num_literal::LiteralNumState::new();
            if num_state.check(env, word) {
                return MatchResult::ContinueWith(word.pos, Box::new(num_state));
            }
        }
        self.match_alias(env, word, rest)
    }

    fn step_match(
        &mut self,
        env: &mut Enviroment,
        did_child_match: bool,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if did_child_match {
            // child matched successfully
            MatchResult::Matched(word.pos)
        } else {
            // child did not match - continue searching
            self.match_alias(env, word, rest)
        }
    }

    fn get_name(&self) -> &'static str {
        self.data.state_name
    }

    fn do_replace(&self) -> bool {
        true
    }
}

impl<'a> NoneState<'a> {
    pub fn new(data: &'a BuiltinData) -> Self {
        let length = data.names.len();
        Self {
            data,
            progress: vec![0u8; length],
            locs: vec![Some(Vec::new()); length],
            offset: 0,
            matched: 0,
        }
    }
    fn reset(&mut self) {
        let length = self.data.names.len();
        self.progress = vec![0u8; length];
        self.locs = vec![Some(Vec::new()); length];
        self.offset = 0;
        self.matched = 0;
    }
    pub fn new_stat(env: &'a Enviroment) -> Self {
        Self::new(&env.aliases.stat)
    }
    pub fn new_expr(env: &'a Enviroment) -> Self {
        Self::new(&env.aliases.expr)
    }
    pub fn new_expr_cont(env: &'a Enviroment) -> Self {
        Self::new(&env.aliases.expr_cont)
    }
}

impl<'a> NoneState<'a> {
    /// matches buildin functions based on self.data
    fn match_alias(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        // run until end of word
        for offset in self.offset..word.len() {
            self.match_letters(word, offset);

            // try match
            while self.matched != 0 {
                self.matched -= 1;
                return self.find_best_match(env, offset, rest.pos);
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

    fn find_best_match(&mut self, env: &mut Enviroment, offset: usize, rest: usize) -> MatchResult {
        let mut min_size = usize::MAX;
        let mut min_locations = usize::MAX;
        let mut min_index = u16::MAX;
        for j in 0..self.data.names.len() {
            // has finished matching
            if self.progress[j] == self.data.names[j].len() as u8 && self.locs[j].is_some() {
                let matching_locs = self.locs[j].as_ref().unwrap();

                let size = matching_locs.last().unwrap() - matching_locs[0];
                let location_sum: usize = matching_locs.iter().sum();

                // is best match
                // match on least length between firt and last letter
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
            min_index, rest,
            // move locs out of state without copy
        )
    }

    fn match_letters(&mut self, word: &Slice<'_>, offset: usize) {
        // does letter match any commands
        for i in 0..self.data.names.len() {
            // does letter match
            if self.progress[i] < self.data.names[i].len() as u8
                && word.str[offset].to_ascii_lowercase()
                    == self.data.names[i][self.progress[i] as usize]
            {
                self.progress[i] += 1;
                // add locations to locations (locs)
                self.locs[i].as_mut().unwrap().push(word.pos + offset);
                if self.progress[i] == self.data.names[i].len() as u8 {
                    self.matched += 1;
                }
            }
        }
    }
}

// fn step_stat(
//     env: &mut Enviroment,
//     result: MatchChildResult,
//     word: &Slice,
//     rest: &Slice,
// ) -> MatchResult {
//     match_built_in(&STAT_DATA, env, result, word, rest)
// }
// fn step_expr(
//     env: &mut Enviroment,
//     result: MatchChildResult,
//     word: &Slice,
//     rest: &Slice,
// ) -> MatchResult {
//     if result == MatchChildResult::None {
//         let var_result = step_var(env, result, word, rest);
//         if matches!(var_result, MatchResult::Matched(_)) {
//             return var_result;
//         }
//     }

//     match_built_in(&EXPR_DATA, env, result, word, rest)
// }
