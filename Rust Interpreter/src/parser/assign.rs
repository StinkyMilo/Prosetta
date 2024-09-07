use super::*;
/// state for equals
#[derive(Debug)]
pub struct AssignState;
impl ParseState for AssignState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        // set expr
        env.exprs.vec[env.index] = Expr::Assign {
            name_start: word.pos + env.global_index,
            name: word.str.to_owned(),
            value_index: usize::MAX,
            locs: env.locs.take().unwrap_or_default(),
            end: End::none(),
        };
        // setup child state
        MatchResult::ContinueWith(rest.pos, Box::new(alias::NoneState::new_expr_cont()))
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<usize>,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if let Some(index) = child_index {
            // find ending close
            let close = find_close_slice(&word, 0).or_else(|| find_close_slice(&rest, 0));
            match close {
                // will never be a period to find even on future words
                None => MatchResult::Failed,
                Some(slice) => {
                    if let Expr::Assign {
                        name,
                        value_index,
                        end,
                        ..
                    } = &mut env.exprs.vec[env.index]
                    {
                        *value_index = index;
                        *end = End::from_slice(&slice.0, env.global_index);
                        env.vars.insert(name.to_owned());
                    } else {
                        unreachable!();
                    }
                    MatchResult::Matched(slice.0.pos, true)
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
