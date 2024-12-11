use std::fmt::Debug;

use super::*;

/// a super state that wants between n and m arguments and a close
pub trait BasicState {
    /// Can this be used here. True by default
    fn can_happen(&self, _env: &mut Environment) -> bool {
        true
    }

    /// get the name
    fn get_name(&self) -> &'static str;

    /// set expr and return whether it is first
    fn do_first(&mut self, expr: &mut Expr, locs: Vec<usize>) -> bool;

    /// add children at index to self
    fn add_child(&mut self, expr: &mut Expr, index: usize, return_type: ReturnType);

    /// can I be closed
    fn can_close(&self) -> CloseType;

    /// set end to index
    fn set_end(&mut self, expr: &mut Expr, index: End);

    fn get_state_return(&self) -> ReturnType;

    fn get_child_type(&self) -> Types;
}

impl<T: BasicState + Debug> ParseState for T {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        if !self.can_happen(env) {
            MatchResult::Failed
        } else {
            let is_first = self.do_first(env.expr, env.locs.take().unwrap_or_default());
            if is_first {
                let can_close = self.can_close();
                match can_close {
                    CloseType::Unable => {
                        // cont - has required arguments
                        MatchResult::ContinueWith(
                            word.pos,
                            self.get_child_type(),
                            get_state!(alias::NoneState::new_expr_cont()),
                        )
                    }
                    CloseType::Able => {
                        if is_mandatory_close(word) {
                            self.set_end(env.expr, End::from_slice(&word, env.global_index));
                            MatchResult::Matched(word.pos, self.get_state_return(), true)
                            // succeeded - continue again with noncont expr
                        } else {
                            MatchResult::ContinueWith(
                                word.pos,
                                self.get_child_type(),
                                get_state!(alias::NoneState::new_expr()),
                            )
                        }
                    }
                    CloseType::Force => {
                        let close =
                            find_close_slice(&word, 0).or_else(|| find_close_slice(&rest, 0));
                        match close {
                            // will never be a period to find even on future words
                            None => MatchResult::Failed,
                            Some(slice) => {
                                self.set_end(env.expr, End::from_slice(&slice.0, env.global_index));
                                MatchResult::Matched(slice.0.pos, self.get_state_return(), true)
                            }
                        }
                    }
                }
            } else {
                // not cont - may have more arguments but may not - need to find close if there
                MatchResult::ContinueWith(
                    word.pos,
                    self.get_child_type(),
                    get_state!(alias::NoneState::new_expr()),
                )
            }
        }
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<(usize, ReturnType)>,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if let Some((index, return_type)) = child_index {
            self.add_child(env.expr, index, return_type);
        }

        let can_close = self.can_close();

        match can_close {
            CloseType::Unable => {
                if child_index.is_some() {
                    // continue again
                    MatchResult::ContinueWith(
                        word.pos,
                        self.get_child_type(),
                        get_state!(alias::NoneState::new_expr_cont()),
                    )
                } else {
                    // exprcont failed on the entire rest of string - I will never match
                    MatchResult::Failed
                }
            }
            CloseType::Able => {
                // I can close so I close
                if is_mandatory_close(word) {
                    self.set_end(env.expr, End::from_slice(&word, env.global_index));
                    MatchResult::Matched(word.pos, self.get_state_return(), true)
                    // succeeded - continue again with noncont expr
                } else if child_index.is_some() {
                    MatchResult::ContinueWith(
                        word.pos,
                        self.get_child_type(),
                        get_state!(alias::NoneState::new_expr()),
                    )
                    // failed - pass word
                } else {
                    MatchResult::Continue(0)
                }
            }
            CloseType::Force => {
                // forced to close
                let close = find_close_slice(&word, 0).or_else(|| find_close_slice(&rest, 0));
                match close {
                    // will never be a period to find even on future words
                    None => MatchResult::Failed,
                    Some(slice) => {
                        self.set_end(env.expr, End::from_slice(&slice.0, env.global_index));
                        MatchResult::Matched(slice.0.pos, self.get_state_return(), true)
                    }
                }
            }
        }
    }

    fn get_name(&self) -> &'static str {
        <Self as BasicState>::get_name(&self)
    }

    fn get_type(&self) -> StateType {
        match self.get_state_return() {
            ReturnType::Void => StateType::Stat,
            _ => StateType::Expr,
        }
    }
}
