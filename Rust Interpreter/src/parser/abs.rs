use super::*;

#[derive(Debug)]
pub struct AbsState;

impl ParseState for AbsState {
    fn step(&mut self, env: &mut Environment, word: &Slice, _rest: &Slice) -> MatchResult {
        // set expr
        *env.expr = Expr::Abs {
            locs: env.locs.take().unwrap_or_default(),
            index: usize::MAX,
            end: End::none(),
        };

        // setup child state
        MatchResult::ContinueWith(
            word.pos,
            Types::Number,
            Box::new(alias::NoneState::new_expr_cont()),
        )
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<(usize, ReturnType)>,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if let Some((ind, _)) = child_index {
            // find ending close
            let close = find_close_slice(&word, 0).or_else(|| find_close_slice(&rest, 0));
            match close {
                // will never be a period to find even on future words
                None => MatchResult::Failed,
                Some(slice) => {
                    if let Expr::Abs { index, end, .. } = env.expr {
                        *index = ind;
                        *end = End::from_slice(&slice.0, env.global_index);
                    } else {
                        unreachable!();
                    }
                    MatchResult::Matched(slice.0.pos, ReturnType::Number, true)
                }
            }
        } else {
            // child expr failed
            // if child match fail, I can never succeed
            MatchResult::Failed
        }
    }
    fn get_name(&self) -> &'static str {
        "Abs"
    }

    fn get_type(&self) -> StateType {
        StateType::Expr
    }
}

impl AbsState {
    pub fn new() -> Self {
        Self
    }
}
