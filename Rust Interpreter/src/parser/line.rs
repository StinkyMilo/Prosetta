use super::*;

#[derive(Debug)]
struct LineState{
    children: u8
}
impl ParseState for LineState{
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        todo!()
    }

    fn step_match(
        &mut self,
        env: &mut Enviroment,
        did_child_match: bool,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        todo!()
    }

    fn get_name(&self) -> &'static str {
       "Line"
    }

    fn do_replace(&self) -> bool {
        false
    }
}
 