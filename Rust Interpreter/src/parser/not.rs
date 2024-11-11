use super::*;

/// state for num
#[derive(Debug)]
pub struct NotState;
impl ParseState for NotState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        // wait for non . word to start
        if is_close(word) {
            MatchResult::Continue(0)
        } else {
            // find close
            let close = find_close_slice(rest, 0);
            //close exists - match
            if let Some(close) = close {
                *env.expr = Expr::Not {
                    locs: env.locs.take().unwrap_or_default(),
                    str_start: word.pos + env.global_index,
                    str_len: word.len(),
                    word:word.str.to_ascii_lowercase(),
                    end: End::from_slice(&close.0, env.global_index),
                };
                env.nots.insert(word.str.to_ascii_lowercase());
                MatchResult::Matched(close.0.pos, ReturnType::Void, true)
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
        "Num"
    }

    fn get_type(&self) -> StateType {
        StateType::Expr
    }
}
impl NotState {
    pub fn new() -> Self {
        Self
    }
}
