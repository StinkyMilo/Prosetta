use super::*;
/// state for equals
#[derive(Debug)]
pub struct WhileState {
    has_condition: bool,
}
impl ParseState for WhileState {
    fn step(&mut self, env: &mut Environment, word: &Slice, _rest: &Slice) -> MatchResult {
        if !self.has_condition {
            *env.expr = Expr::While {
                locs: env.locs.take().unwrap_or_default(),
                indexes: Vec::new(),
                end: End::none(),
            };
            env.vars.add_layer();
            // setup child state
            MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_expr_cont()))
        } else {
            MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_stat()))
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
                    MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_stat()))
                } else {
                    // if child match fail, I can never succeed
                    MatchResult::Failed
                }
            } else {
                //and stat child
                if let Some(index) = child_index {
                    indexes.push(index);
                }

                // close if have close
                if is_close(word) {
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

    fn do_replace(&self) -> bool {
        false
    }
}

impl WhileState {
    pub fn new() -> Self {
        Self {
            has_condition: false,
        }
    }
}