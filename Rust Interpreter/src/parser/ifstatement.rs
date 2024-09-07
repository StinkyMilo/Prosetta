use super::*;
/// state for equals
#[derive(Debug)]
pub struct IfState {
    has_condition: bool,
}
impl ParseState for IfState {
    fn step(&mut self, env: &mut Environment, word: &Slice, _rest: &Slice) -> MatchResult {
        if !self.has_condition {
            *env.expr = Expr::If {
                locs: env.locs.take().unwrap_or_default(),
                // condition_start: word.pos + env.global_index,
                // body_start: usize::MAX,
                indexes: Vec::new(),
                end: End::none(),
            };
            // setup child state
            MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_expr_cont()))
        } else {
            //println!("Continuing with new statement");
            MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_stat_cont()))
        }
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<usize>,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        let mut has_else = false;
        if self.has_condition {
            if let Some(_index) = child_index {
                has_else = matches!(env.expr, Expr::Else { .. });
            }
        }
        if let Expr::If {
            // body_start,
            // body_end,
            indexes,
            ..
        } = env.expr
        {
            //If we get a punctuation before an expression, we want to end. Otherwise, we want to continue with a new expression
            //Check the next close. Is it after the child expression? If so, don't even add the child and fail.
            if !(self.has_condition) {
                if let Some(index) = child_index {
                    indexes.push(index);
                    self.has_condition = true;
                    //*body_start = index;
                    MatchResult::ContinueWith(
                        word.pos,
                        get_state!(alias::NoneState::new_stat_cont()),
                    )
                } else {
                    //No child
                    MatchResult::Failed
                }
            } else {
                let mut statement_found = false;
                if let Some(index) = child_index {
                    indexes.push(index);
                    statement_found = true;
                }

                if is_close(word) {
                    //*body_end = word.pos + env.global_index;
                    MatchResult::Matched(word.pos, true)
                } else if statement_found {
                    if has_else {
                        //Else must be the last statement
                        let close = find_close(&word, 0).or_else(|| find_close(&rest, 0));
                        match close {
                            // will never be a period to find even on future words
                            None => MatchResult::Failed,
                            Some(slice) => {
                                //*body_end = slice.pos + env.global_index;
                                MatchResult::Matched(slice.pos, true)
                            }
                        }
                    } else {
                        MatchResult::ContinueWith(
                            word.pos,
                            get_state!(alias::NoneState::new_stat_cont()),
                        )
                    }
                } else {
                    MatchResult::Continue
                }
            }
        } else {
            MatchResult::Failed
        }
    }

    fn get_name(&self) -> &'static str {
        "If"
    }

    fn do_replace(&self) -> bool {
        false
    }
}

impl IfState {
    pub fn new() -> Self {
        Self {
            has_condition: false,
        }
    }
}
