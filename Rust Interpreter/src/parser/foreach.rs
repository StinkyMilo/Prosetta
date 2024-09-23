use super::*;
/// state for equals
#[derive(Debug)]
pub struct ForEachState {
    first: bool,
    has_list: bool,
    has_stat: bool,
}
impl ParseState for ForEachState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        if self.first {
            *env.expr = Expr::ForEach {
                name_start: usize::MAX,
                name: Vec::new(),
                locs: env.locs.take().unwrap_or_default(),
                indexes: Vec::new(),
                end: End::none(),
            };
            self.first = false;
            // setup child state
            // MatchResult::ContinueWith(rest.pos, Box::new(alias::NoneState::new_expr_cont()))
        } 
        if is_close(word) || (word.len() > 0 && (word.str[0] == b'"' || word.str[0] == b'\'')) {
            MatchResult::Continue
        }else if !self.has_list{
            if let Expr::ForEach { 
                name_start, name, ..
            } = env.expr {
                *name_start = word.pos + env.global_index;
                *name = word.str.to_owned();
                env.vars.add_layer();
                env.vars.insert(name.to_ascii_lowercase().to_owned());
            } else{
                unreachable!()
            }
            MatchResult::ContinueWith(rest.pos, Box::new(alias::NoneState::new_expr_cont()))
        }else if self.has_stat {
            MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_stat()))
        } else {
            MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_stat_cont()))
        }
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<usize>,
        word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        if let Expr::ForEach { indexes, end, .. } = env.expr {
            if !self.has_list {
                //add child and find stats
                if let Some(index) = child_index {
                    self.has_list = true;
                    indexes.push(index);
                    MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_stat_cont()))
                } else {
                    // if child match fail, I can never succeed
                    MatchResult::Failed
                }
            } else {
                //and stat child
                if let Some(index) = child_index {
                    self.has_stat = true;
                    indexes.push(index);
                }

                // close if have close
                if self.has_stat && is_close(word) {
                    *end = End::from_slice(&word, env.global_index);
                    env.vars.remove_layer();
                    MatchResult::Matched(word.pos, true)
                    // succeeded - continue again with noncont stat
                } else if child_index.is_some() {
                    MatchResult::ContinueWith(word.pos, get_state!(alias::NoneState::new_stat()))
                    // failed - pass word
                } else {
                    MatchResult::Continue
                }
            }
        } else {
            unreachable!()
        }
    }

    fn get_name(&self) -> &'static str {
        "While"
    }

    fn get_type(&self) -> StateType {
        StateType::Stat
    }
}

impl ForEachState {
    pub fn new() -> Self {
        Self {
            first: true,
            has_list: false,
            has_stat: false,
        }
    }
}
