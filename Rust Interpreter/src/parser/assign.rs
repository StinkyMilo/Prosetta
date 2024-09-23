use std::usize;

use super::*;
/// state for equals
#[derive(Debug)]
pub struct AssignState {
    first: bool,
}
impl ParseState for AssignState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        // set expr
        if self.first {
            *env.expr = Expr::Assign {
                name_start: usize::MAX,
                name: Vec::new(),
                value_index: usize::MAX,
                locs: env.locs.take().unwrap_or_default(),
                end: End::none(),
                first: false,
            };
            self.first = false;
        }

        // dont make closes varibles
        if is_close(word) || (word.len() > 0 && (word.str[0] == b'"' || word.str[0] == b'\'')) {
            MatchResult::Continue
        } else {
            //set name
            if let Expr::Assign {
                name_start,
                name,
                first,
                ..
            } = env.expr
            {
                let var = word.str.to_ascii_lowercase();
                *name_start = word.pos + env.global_index;
                *first = !env.vars.contains(&var);
                *name = var;
            } else {
                unreachable!()
            }
            // setup child state
            MatchResult::ContinueWith(rest.pos, Box::new(alias::NoneState::new_expr_cont()))
        }
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
                    } = env.expr
                    {
                        *value_index = index;
                        *end = End::from_slice(&slice.0, env.global_index);
                        env.vars.insert(name.to_ascii_lowercase().to_owned());
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

    fn get_type(&self) -> StateType {
        StateType::Stat
    }
}

impl AssignState {
    pub fn new() -> Self {
        Self { first: true }
    }
}
