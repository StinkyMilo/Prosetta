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
                var: SubStrData::new(),
                value_index: usize::MAX,
                locs: env.locs.take().unwrap_or_default(),
                end: End::none(),
                first: false,
            };
            self.first = false;
        }

        // dont make closes varibles
        let var_word = try_get_symbol_word(word, env.global_index);
        if let Some(new_var) = var_word {
            //set name
            if let Expr::Assign { var, first, .. } = env.expr {
                *first = !env.symbols.contains(&new_var.name);
                *var = new_var;
            } else {
                unreachable!()
            }
            // setup child state
            MatchResult::ContinueWith(
                rest.pos,
                Types::Any,
                Box::new(alias::NoneState::new_expr_cont()),
            )
        } else {
            MatchResult::Continue(0)
        }
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<(usize, ReturnType)>,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if let Some((index, return_type)) = child_index {
            // find ending close
            let close = find_close_slice(&word, 0).or_else(|| find_close_slice(&rest, 0));
            match close {
                // will never be a period to find even on future words
                None => MatchResult::Failed,
                Some(slice) => {
                    if let Expr::Assign {
                        var,
                        value_index,
                        end,
                        ..
                    } = env.expr
                    {
                        *value_index = index;
                        *end = End::from_slice(&slice.0, env.global_index);
                        env.symbols.insert_var(var.name.to_owned(), return_type);
                    } else {
                        unreachable!();
                    }
                    MatchResult::Matched(slice.0.pos, ReturnType::Void, true)
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
