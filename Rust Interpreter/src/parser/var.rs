use super::*;
/// state for equals
#[derive(Debug)]
pub struct VarState;
impl ParseState for VarState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        // get lowercase
        let lower = word.str.to_ascii_lowercase();

        // is varible in scope
        if env.vars.contains(lower.clone()) {
            *env.expr = Expr::Var {
                name_start: word.pos + env.global_index,
                name: lower,
            };
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
        "Var"
    }

    fn do_replace(&self) -> bool {
        false
    }
}
impl VarState {
    pub fn new() -> Self {
        Self
    }
}
