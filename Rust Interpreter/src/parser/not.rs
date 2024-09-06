use super::*;

#[derive(Debug)]

pub struct NotState;

impl ParseState for NotState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        let close = find_close(&word, 0).or_else(|| find_close(&rest, 0));
        if let Some(index) = close {
            env.exprs.vec[env.index] = Expr::Skip {
                locs: env.locs.take().unwrap_or_default(),
                index: usize::MAX,
                start: word.pos + env.global_index,
                end: index.pos + env.global_index,
            };

            MatchResult::ContinueWith(index.pos + 1, Box::new(alias::NoneState::new_expr_cont()))
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
            if let Expr::Skip { index, .. } = &mut env.exprs.vec[env.index] {
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
