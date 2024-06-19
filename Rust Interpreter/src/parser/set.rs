use super::*;
/// state for equals
#[derive(Debug)]
pub struct EqState {}
impl ParseState for EqState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        // set expr
        *env.expr = Expr::Set {
            name_start: word.pos + env.global_index,
            name: word.str.to_owned(),
            value_index: env.child_index,
            locs: env.locs.take().unwrap_or_default(),
        };
        // setup child state
        MatchResult::ContinueWith(rest.pos, Box::new(alias::NoneState::new_expr_cont()))
    }

    fn step_match(
        &mut self,
        _env: &mut Enviroment,
        did_child_match: bool,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if did_child_match {
            // find ending close
            let close = find_close(&word, 0).or_else(|| find_close(&rest, 0));
            match close {
                // will never be a period to find even on future words
                None => MatchResult::Failed,
                Some(slice) => MatchResult::Matched(slice.pos),
            }
        } else {
            // child expr failed
            // if child match fail, I can never succeed
            MatchResult::Failed
        }
    }

    fn get_name(&self) -> &'static str {
        "Equals"
    }

    fn do_replace(&self) -> bool {
        false
    }
}

impl EqState {
    pub fn new() -> Self {
        Self {}
    }
}
