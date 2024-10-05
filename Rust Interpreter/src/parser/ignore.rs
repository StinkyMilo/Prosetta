use super::*;
/// state for equals
#[derive(Debug)]
pub struct IgnoreState;
impl ParseState for IgnoreState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        // get lowercase
        let lower = word.str.to_ascii_lowercase();

        // is varible in scope
        if let Some((index, name)) = env.nots.try_get_val(&lower) {
            *env.expr = Expr::Ignore {
                name_start: word.pos + env.global_index + index,
                name,
            };
            MatchResult::Matched(rest.pos, false)
        } else {
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
