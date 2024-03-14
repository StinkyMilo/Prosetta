use super::*;
use builtins_data::*;

/// used for both NoneStat and NoneExpr
/// finds next buildin function
#[derive(Debug)]
pub struct NoneState {
    data: &'static BuiltinData,
    progress: Vec<u8>,
    locs: Vec<Option<Vec<usize>>>,
    offset: usize,
    matched: u16,
}

impl ParseState for NoneState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        debug_assert!(self.data.names.len() < u16::MAX as usize);
        // if expr  - check if varible name
        if self.data.is_expr {
            let var = match_var(env, word, rest);
            if let Some(end) = var {
                return MatchResult::Matched(end);
            }
        }
        self.match_built_in(env, word, rest)
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
            self.match_built_in(env, word, rest)
        }
    }

    fn get_name(&self) -> &'static str {
        let expr = if self.data.is_expr {
            Expr::NoneExpr
        } else {
            Expr::NoneStat
        };
        expr.get_name()
    }
    fn do_replace(&self) -> bool {
        true
    }
}

impl NoneState {
    fn new(data: &'static BuiltinData) -> Self {
        let length = data.names.len();
        Self {
            data,
            progress: vec![0u8; length],
            locs: vec![Some(Vec::new()); length],
            offset: 0,
            matched: 0,
        }
    }
    pub fn new_stat() -> Self {
        Self::new(&STAT_DATA)
    }
    pub fn new_expr() -> Self {
        Self::new(&EXPR_DATA)
    }
}

impl NoneState {
    /// matches buildin functions based on self.data
    fn match_built_in(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        // run until end of word
        for offset in self.offset..word.len() {
            self.match_letters(word, offset);

            // try match
            while self.matched != 0 {
                self.matched -= 1;
                return self.find_best_match(env,offset, rest.pos);
            }
        }
        // try match next word
        MatchResult::ContinueFail
    }

    fn find_best_match(&mut self,env: &mut Enviroment, offset: usize,  rest: usize) -> MatchResult {
        let mut min_size = usize::MAX;
        let mut min_locations = usize::MAX;
        let mut min_index = u16::MAX;
        for j in 0..self.data.names.len() {
            // has finished matching
            if self.progress[j] == self.data.names[j].len() as u8 {
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
        env.locs =  self.locs[min_index as usize].take();
        //set up stack
        (self.data.func)(
            min_index,
            rest,
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

fn match_var(env: &mut Enviroment, word: &Slice, rest: &Slice) -> Option<usize> {
    // is varible in scope
    if env.vars.contains(word.str) {
        *env.expr = Expr::Var {
            name_start: word.pos,
            name: word.str.to_owned(),
        };
        Some(rest.pos)
    } else {
        // future words could be varible names
        None
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
