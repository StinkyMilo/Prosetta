use super::*;
/// state for equals
#[derive(Debug)]
pub struct IgnoreState;
impl ParseState for IgnoreState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        // is varible in scope
        if let Some(data) = env.nots.try_get_val(&word, env.global_index) {
            *env.expr = Expr::Ignore { data };
            MatchResult::Matched(rest.pos, false)
        } else {
            // future words could be varible names
            MatchResult::Failed
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
        "Ignore"
    }

    fn get_type(&self) -> StateType {
        StateType::Expr
    }
}
impl IgnoreState {
    pub fn new() -> Self {
        Self
    }
}
