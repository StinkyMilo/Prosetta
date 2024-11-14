use super::*;
/// state for equals
#[derive(Debug)]
pub struct FrameState;
impl ParseState for FrameState {
    fn step(&mut self, env: &mut Environment, word: &Slice, _rest: &Slice) -> MatchResult {
        *env.expr = Expr::Frame {
            locs: env.locs.take().unwrap_or_default(),
        };
        MatchResult::Matched(word.pos, ReturnType::Number, false)
    }

    fn step_match(
        &mut self,
        _env: &mut Environment,
        _child_index: Option<(usize, ReturnType)>,
        _word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        // has no child to match - fn should never be called
        unreachable!()
    }

    fn get_name(&self) -> &'static str {
        "Frame"
    }

    fn get_type(&self) -> StateType {
        StateType::Expr
    }
}
impl FrameState {
    pub fn new() -> Self {
        Self
    }
}
