use super::*;
/// state for equals
#[derive(Debug)]
pub struct FunctionCallState {
    is_first: bool,
    count: usize,
}
impl ParseState for FunctionCallState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        // get lowercase
        if self.is_first {
            self.is_first = false;
            // is varible in scope
            if let Some(func) = env.funcs.try_get_func(&word, env.global_index) {
                *env.expr = Expr::FunctionCall {
                    locs: env.locs.take().unwrap_or_default(),
                    func,
                    indexes: Vec::new(),
                    end: End::none(),
                };
                MatchResult::ContinueWith(rest.pos, get_state!(alias::NoneState::new_expr_cont()))
            } else {
                // future words could be varible names
                MatchResult::Failed
            }
        } else {
            MatchResult::ContinueWith(word.pos, get_state!(alias::NoneState::new_expr()))
        }
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<usize>,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if let Expr::FunctionCall {
            func, indexes, end, ..
        } = env.expr
        {
            if let Some(index) = child_index {
                indexes.push(index);
                self.count += 1;
            }
            if let Some(arg_total) = env.funcs.get_arg_count(&func.name) {
                let can_close = self.count >= *arg_total;
                // println!(
                //     "[OUTPUT_TEST] count: {}, name: {}, arg total: {}",
                //     self.count,
                //     String::from_utf8_lossy(&name.to_vec()),
                //     *arg_total
                // );
                if can_close {
                    let close = find_close_slice(&word, 0).or_else(|| find_close_slice(&rest, 0));
                    return match close {
                        None => MatchResult::Failed,
                        Some(slice) => {
                            *end = End::from_slice(&slice.0, env.global_index);
                            MatchResult::Matched(slice.0.pos, true)
                        }
                    };
                } else {
                    return if child_index.is_some() {
                        MatchResult::ContinueWith(
                            word.pos,
                            get_state!(alias::NoneState::new_expr_cont()),
                        )
                    } else {
                        MatchResult::Failed
                    };
                }
            } else {
                unreachable!()
            }
        }
        MatchResult::Failed
    }

    fn get_name(&self) -> &'static str {
        "FunctionCall"
    }

    fn get_type(&self) -> StateType {
        StateType::Expr
    }
}
impl FunctionCallState {
    pub fn new() -> Self {
        Self {
            is_first: true,
            count: 0,
        }
    }
}
