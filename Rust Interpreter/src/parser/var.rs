use super::*;
/// state for equals
#[derive(Debug)]
pub struct VarState;
impl ParseState for VarState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        // is varible in scope
        if let Some(var) = env.vars.try_get_var(&word, env.global_index) {
            *env.expr = Expr::Var { var };
            MatchResult::Matched(rest.pos, false)
        } else {
            // future words could be variable names
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
        "Var"
    }

    fn get_type(&self) -> StateType {
        StateType::Expr
    }
}
impl VarState {
    pub fn new() -> Self {
        Self
    }
}
