use super::*;

/// state for num
#[derive(Debug)]
pub struct WordNumState;
impl ParseState for WordNumState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        // wait for non . word to start
        if is_close(word) {
            MatchResult::Continue
        } else {
            // find close
            let close = find_close_slice(rest, 0);
            //close exists - match
            if let Some(index) = close {
                *env.expr = Expr::WordNum {
                    locs: env.locs.take().unwrap_or_default(),
                    str_start: word.pos + env.global_index,
                    str_len: word.len(),
                    end: End::from_slice(&index.0, env.global_index),
                };

                MatchResult::Matched(index.0.pos, true)
            } else {
                // did not find close - fail
                MatchResult::Failed
            }
        }
    }

    fn step_match(
        &mut self,
        _env: &mut Enviroment,
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

    fn do_replace(&self) -> bool {
        false
    }
}
impl WordNumState {
    pub fn new() -> Self {
        Self
    }
}
