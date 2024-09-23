use super::*;
/// state for equals
#[derive(Debug)]
pub struct WhileState {
    has_condition: bool,
    has_stat: bool,
}
impl ParseState for WhileState {
    fn step(&mut self, env: &mut Environment, word: &Slice, _rest: &Slice) -> MatchResult {
        if !self.has_condition {
            *env.expr = Expr::While {
                locs: env.locs.take().unwrap_or_default(),
                indexes: Vec::new(),
                end: End::none(),
            };
            env.add_var_layer();
            // setup child state
            MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_expr_cont()))
        } else if self.has_stat {
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
        if let Expr::While { indexes, end, .. } = env.expr {
            if !self.has_condition {
                //add child and find stats
                if let Some(index) = child_index {
                    self.has_condition = true;
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
                    env.remove_var_layer();
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

impl WhileState {
    pub fn new() -> Self {
        Self {
            has_condition: false,
            has_stat: false,
        }
    }
}
