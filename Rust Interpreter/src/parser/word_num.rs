use std::usize;

use super::*;

/// state for num
#[derive(Debug)]
pub struct WordNumState {
    first: bool,
}
impl ParseState for WordNumState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        if self.first {
            *env.expr = Expr::WordNum {
                locs: env.locs.take().unwrap_or_default(),
                str_start: usize::MAX,
                str_len: 0,
                end: End::none(),
            };
            self.first = false;
        }

        // wait for non . word to start
        if is_close(word) {
            MatchResult::Continue(0)
        } else {
            // find close
            let close = find_close_slice(rest, 0);
            //close exists - match
            if let Some(close) = close {
                if let Expr::WordNum {
                    str_start,
                    str_len,
                    end,
                    ..
                } = env.expr
                {
                    *str_start = word.pos + env.global_index;
                    *str_len = word.len();
                    *end = End::from_slice(&close.0, env.global_index);
                }

                MatchResult::Matched(close.0.pos, true)
            } else {
                // did not find close - fail
                MatchResult::Failed
            }
        }
    }

    fn step_match(
        &mut self,
        _env: &mut Environment,
        _child_index: Option<usize>,
        _word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        // has no child to match - fn should never be called
        unreachable!()
    }

    fn get_name(&self) -> &'static str {
        "WordNum"
    }

    fn get_type(&self) -> StateType {
        StateType::Expr
    }
}
impl WordNumState {
    pub fn new() -> Self {
        Self { first: true }
    }
}
