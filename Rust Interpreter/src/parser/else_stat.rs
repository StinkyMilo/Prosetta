use std::usize;

use super::*;
/// state for equals
#[derive(Debug)]
pub struct ElseState {
    first: bool,
    if_index: usize,
}
impl ParseState for ElseState {
    fn step(&mut self, env: &mut Environment, word: &Slice, _rest: &Slice) -> MatchResult {
        if self.first {
            //get some(index) if it is an "if"
            let if_index = env
                .last_matched_index
                .and_then(|index| matches!(env.parents[index], Expr::If { .. }).then_some(index));

            if let Some(index) = if_index {
                *env.expr = Expr::Else {
                    locs: env.locs.take().unwrap_or_default(),
                    indexes: Vec::new(),
                    end: End::none(),
                };
                // go up layer
                env.vars.add_layer();
                self.if_index = index;
            } else {
                return MatchResult::Failed;
            }
        }
        // non cont stat for seeing closes
        MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_stat()))
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<usize>,
        word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        self.first = false;
        if let Expr::If { else_index, .. } = &mut env.parents[self.if_index] {
            if let Expr::Else { end, indexes, .. } = env.expr {
                if let Some(index) = child_index {
                    indexes.push(index);
                }

                // close if have close
                if is_close(word) {
                    *end = End::from_slice(&word, env.global_index);
                    *else_index = env.expr_index;
                    env.vars.remove_layer();
                    MatchResult::Matched(word.pos, true)
                    // succeeded - continue again with noncont stat
                } else if child_index.is_some() {
                    MatchResult::ContinueWith(word.pos, get_state!(alias::NoneState::new_stat()))
                    // failed - pass word
                } else {
                    MatchResult::Continue
                }
            } else {
                unreachable!()
            }
        } else {
            // should be guarenteed that if_index points to if
            unreachable!()
        }
    }

    fn get_name(&self) -> &'static str {
        "Else"
    }

    fn get_type(&self) -> StateType {
        StateType::Stat
    }
}

impl ElseState {
    pub fn new() -> Self {
        Self {
            first: true,
            if_index: usize::MAX,
        }
    }
}
