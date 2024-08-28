use std::fmt::Debug;

use super::*;

/// a super state that wants between n and m arguments and a close
pub trait BasicState {
    /// get the name
    fn get_name(&self) -> &'static str;

    /// set expr and return whether it is first
    fn do_first(&self, expr: &mut Expr, locs: Vec<usize>) -> bool;

    /// add children at index to self
    fn add_child(&mut self, expr: &mut Expr, index: usize);

    /// can I be closed
    fn can_close(&self) -> CloseType;

    /// set end to index
    fn set_end(&mut self, expr: &mut Expr, index: usize);
}

impl<T: BasicState + Debug> ParseState for T {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, _rest: &Slice) -> MatchResult {
        let is_first = self.do_first(env.expr, env.locs.take().unwrap_or_default());
        if is_first {
            // cont - has required arguments
            MatchResult::ContinueWith(word.pos, get_state!(alias::NoneState::new_expr_cont()))
        } else {
            // not cont - may have more arguments but may not - need to find close if there
            MatchResult::ContinueWith(word.pos, get_state!(alias::NoneState::new_expr()))
        }
    }

    fn step_match(
        &mut self,
        env: &mut Enviroment,
        child: Option<usize>,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if let Some(index) = child {
            self.add_child(env.expr, index);
        }

        let can_close = self.can_close();

        match can_close {
            CloseType::Unable => {
                if child.is_some() {
                    // continue again
                    MatchResult::ContinueWith(
                        word.pos,
                        get_state!(alias::NoneState::new_expr_cont()),
                    )
                } else {
                    // exprcont failed on the entire rest of string - I will never match
                    MatchResult::Failed
                }
            }
            CloseType::Able => {
                // I can close so I close
                if is_close(word) {
                    self.set_end(env.expr, word.pos + env.global_index);
                    MatchResult::Matched(word.pos + 1)
                    // succeeded - continue again with noncont expr
                } else if child.is_some() {
                    MatchResult::ContinueWith(word.pos, get_state!(alias::NoneState::new_expr()))
                    // failed - pass word
                } else {
                    MatchResult::Continue
                }
            }
            CloseType::Force => {
                // forced to close
                let close = find_close(&word, 0).or_else(|| find_close(&rest, 0));
                match close {
                    // will never be a period to find even on future words
                    None => MatchResult::Failed,
                    Some(slice) => {
                        self.set_end(env.expr, slice.pos + env.global_index);
                        MatchResult::Matched(slice.pos + 1)
                    }
                }
            }
        }
    }

    fn get_name(&self) -> &'static str {
        <Self as BasicState>::get_name(&self)
    }

    fn do_replace(&self) -> bool {
        todo!()
    }
}
