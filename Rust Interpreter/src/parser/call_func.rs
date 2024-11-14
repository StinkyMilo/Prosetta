use super::*;
/// state for equals
#[derive(Debug)]
pub struct FunctionCallState {
    // is_first: bool,
    count: u8,
}
impl ParseState for FunctionCallState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        // is varible in scope
        if let Some(func) = env.symbols.try_get_func(&word, env.global_index) {
            // if func exists it should have function arguments
            let arg_count = env.symbols.get_func_arg_count(&func.name).unwrap();

            // if args are 0 then find end
            let end = if arg_count == 0 {
                let close = find_close_slice(&word, 0).or_else(|| find_close_slice(&rest, 0));
                match close {
                    None => return MatchResult::Failed,
                    Some(slice) => End::from_slice(&slice.0, env.global_index),
                }
            } else {
                End::none()
            };

            // set
            *env.expr = Expr::FunctionCall {
                locs: env.locs.take().unwrap_or_default(),
                func,
                indexes: Vec::new(),
                end,
            };

            // match if args are 0
            if arg_count == 0 {
                MatchResult::Matched(end.index - env.global_index, ReturnType::Any, true)
            } else {
                MatchResult::ContinueWith(
                    rest.pos,
                    Types::Any,
                    get_state!(alias::NoneState::new_expr_cont()),
                )
            }
        } else {
            // did not find function
            MatchResult::Failed
        }
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<(usize, ReturnType)>,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if let Expr::FunctionCall {
            func, indexes, end, ..
        } = env.expr
        {
            if let Some((index,_)) = child_index {
                indexes.push(index);
                self.count += 1;
            }
            if let Some(arg_total) = env.symbols.get_func_arg_count(&func.name) {
                let can_close = self.count >= arg_total;
                if can_close {
                    let close = find_close_slice(&word, 0).or_else(|| find_close_slice(&rest, 0));
                    return match close {
                        None => MatchResult::Failed,
                        Some(slice) => {
                            *end = End::from_slice(&slice.0, env.global_index);
                            MatchResult::Matched(slice.0.pos, ReturnType::Any, true)
                        }
                    };
                } else {
                    if child_index.is_some() {
                        MatchResult::ContinueWith(
                            word.pos,
                            Types::Any,
                            get_state!(alias::NoneState::new_expr_cont()),
                        )
                    } else {
                        MatchResult::Failed
                    }
                }
            } else {
                //varible should not have dissapeared from symbols
                unreachable!()
            }
        } else {
            unreachable!()
        }
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
            // is_first: true,
            count: 0,
        }
    }
}
