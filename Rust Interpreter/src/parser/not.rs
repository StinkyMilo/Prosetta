use super::*;

#[derive(Debug)]

pub struct NotState {}

impl ParseState for NotState {
    fn step(&mut self, _env: &mut Enviroment, _word: &Slice, _rest: &Slice) -> MatchResult {
        todo!()
    }

    fn step_match(
        &mut self,
        _env: &mut Enviroment,
        _child_index:Option<usize>,
        _word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        // has no child to match - fn should never be called
        todo!()
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
        NotState {}
    }
}
