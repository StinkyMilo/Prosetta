use std::usize;

use super::*;
/// state for equals
#[derive(Debug)]
pub struct ElseState {
    first: bool,
    if_index: usize,
    has_stat: bool,
}
impl ParseState for ElseState {
    fn step(&mut self, env: &mut Environment, word: &Slice, _rest: &Slice) -> MatchResult {
        if self.first {
            //get some(index) if it is an "if"
            let if_index = env.last_stat_index.and_then(|index| {
                matches!(env.before.get(index), Some(Expr::If { .. })).then_some(index)
            });

            if let Some(index) = if_index {
                if !env
                    .parents
                    .into_iter()
                    .rev()
                    .take_while(|state| state.expr_index > index)
                    .any(|state| state.state.get_type() == StateType::Stat)
                {
                    *env.expr = Expr::Else {
                        locs: env.locs.take().unwrap_or_default(),
                        indexes: Vec::new(),
                        end: End::none(),
                    };
                    // go up layer
                    env.symbols.add_layer();
                    self.if_index = index;
                }
            }
        }
        // if "if" wasnt found
        if self.if_index == usize::MAX {
            MatchResult::Failed
        // non cont stat for seeing closes
        } else if self.has_stat {
            MatchResult::ContinueWith(
                word.pos,
                Types::Void,
                Box::new(alias::NoneState::new_stat()),
            )
            // need a first stat - cont
        } else {
            MatchResult::ContinueWith(
                word.pos,
                Types::Void,
                Box::new(alias::NoneState::new_stat_cont()),
            )
        }
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<usize>,
        word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        self.first = false;
        if let Expr::If { else_index, .. } = &mut env.before[self.if_index] {
            if let Expr::Else { end, indexes, .. } = env.expr {
                if let Some(index) = child_index {
                    self.has_stat = true;
                    indexes.push(index);
                }

                // close if have close
                if is_mandatory_close(word) {
                    *end = End::from_slice(&word, env.global_index);
                    *else_index = env.expr_index;
                    env.symbols.remove_layer();
                    MatchResult::Matched(word.pos, ReturnType::Void, true)
                    // succeeded - continue again with noncont stat
                } else if child_index.is_some() {
                    MatchResult::ContinueWith(
                        word.pos,
                        Types::Void,
                        get_state!(alias::NoneState::new_stat()),
                    )
                    // failed - pass word
                } else {
                    MatchResult::Continue(0)
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
            has_stat: false,
        }
    }
}
