use super::*;

#[derive(Debug)]

pub struct NotState;

impl ParseState for NotState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        let close = find_close_slice(&word, 0).or_else(|| find_close_slice(&rest, 0));
        if let Some(slice) = close {
            *env.expr = Expr::Skip {
                locs: env.locs.take().unwrap_or_default(),
                index: usize::MAX,
                start: word.pos + env.global_index,
                end: End::from_slice(&slice.0, env.global_index),
            };

            MatchResult::ContinueWith(slice.0.pos + 1, Box::new(alias::NoneState::new_expr_cont()))
        } else {
            // no . - will never match
            MatchResult::Failed
        }
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<usize>,
        word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        // child matched - add index of child and match
        if let Some(new_index) = child_index {
            if let Expr::Skip { index, .. } = env.expr {
                *index = new_index;
            };

            MatchResult::Matched(word.pos, false)
        // child failed - I fail
        } else {
            MatchResult::Failed
        }
    }

    fn get_name(&self) -> &'static str {
        "Not"
    }

    fn do_replace(&self) -> bool {
        false
    }
}

impl NotState {
    pub fn new() -> Self {
        Self
    }
}
