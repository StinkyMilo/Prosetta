use super::*;

#[derive(Debug)]

pub struct PrintState {}

impl ParseState for PrintState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        unimplemented!()
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
        "Not"
    }

    fn do_replace(&self) -> bool {
        false
    }
}

impl PrintState {
    pub fn new() -> Self {
        PrintState {}
    }
}
