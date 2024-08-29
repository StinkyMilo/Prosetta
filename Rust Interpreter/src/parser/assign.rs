use super::*;
/// state for equals
#[derive(Debug)]
pub struct AssignState;
impl ParseState for AssignState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        // set expr
        *env.expr = Expr::Assign {
            name_start: word.pos + env.global_index,
            name: word.str.to_owned(),
            value_index: usize::MAX,
            locs: env.locs.take().unwrap_or_default(),
            end: usize::MAX,
        };
        // setup child state
        MatchResult::ContinueWith(rest.pos, Box::new(alias::NoneState::new_expr_cont()))
    }

    fn step_match(
        &mut self,
        env: &mut Enviroment,
        child_index: Option<usize>,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if let Some(index) = child_index {
            // find ending close
            let close = find_close(&word, 0).or_else(|| find_close(&rest, 0));
            match close {
                // will never be a period to find even on future words
                None => MatchResult::Failed,
                Some(slice) => {
                    if let Expr::Assign {
                        value_index, end, ..
                    } = env.expr
                    {
                        *value_index = index;
                        *end = slice.pos + env.global_index;
                    }
                    MatchResult::Matched(slice.pos, true)
                }
            }
        } else {
            // child expr failed
            // if child match fail, I can never succeed
            MatchResult::Failed
        }
    }

    fn get_name(&self) -> &'static str {
        "Assign"
    }

    fn do_replace(&self) -> bool {
        false
    }
}

impl AssignState {
    pub fn new() -> Self {
        Self
    }
}
