use super::*;
/// state for equals
#[derive(Debug)]
pub struct VarState {}
impl ParseState for VarState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        //get lowercase
        let lower = word.str.to_ascii_lowercase();

        // is varible in scope
        if env.vars.contains(&lower) {
            *env.expr = Expr::Var {
                name_start: word.pos + env.global_index,
                name: lower,
            };
            MatchResult::Matched(rest.pos)
        } else {
            // future words could be varible names
            MatchResult::Failed
        }
    }

    fn step_match(
        &mut self,
        _env: &mut Enviroment,
        _did_child_match: bool,
        _word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        // has no child to match - fn should never be called
        unimplemented!()
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
        VarState { }
    }
    // pub fn check(&mut self, env: &mut Enviroment, word: &Slice) -> bool {
    //     self.is_checked = env.vars.contains(&word.str.to_ascii_lowercase());
    //     self.is_checked
    // }
}