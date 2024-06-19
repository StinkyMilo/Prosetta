use super::*;

#[derive(Debug)]

pub struct RectState {}

impl ParseState for RectState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        todo!()
    }

    fn step_match(
        &mut self,
        _env: &mut Enviroment,
        _did_child_match: bool,
        _word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        todo!()
    }

    fn get_name(&self) -> &'static str {
        "Rect"
    }

    fn do_replace(&self) -> bool {
        false
    }
}

impl RectState {
    pub fn new() -> Self {
        RectState {}
    }
}
