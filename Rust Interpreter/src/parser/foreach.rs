use super::*;
/// state for equals
#[derive(Debug)]
pub struct ForEachState {
    first: bool,
    has_list: bool,
    has_stat: bool,
}
impl ParseState for ForEachState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        if self.first {
            *env.expr = Expr::ForEach {
                var: SubStrData::new(),
                locs: env.locs.take().unwrap_or_default(),
                indexes: Vec::new(),
                end: End::none(),
            };
            self.first = false;
            // setup child state
            // MatchResult::ContinueWith(rest.pos, Box::new(alias::NoneState::new_expr_cont()))
        }
        if !self.has_list {
            let var_word = try_get_symbol_word(word, env.global_index);
            if let Some(new_var) = var_word {
                if let Expr::ForEach { var, .. } = env.expr {
                    *var = new_var;
                } else {
                    unreachable!()
                }
                MatchResult::ContinueWith(
                    rest.pos,
                    Types::List | Types::Number,
                    Box::new(alias::NoneState::new_expr_cont()),
                )
            } else {
                MatchResult::Continue(0)
            }
            // reach end of buffer
        } else if word.len() == 0 {
            env.symbols.remove_layer();
            MatchResult::Failed
        } else if self.has_stat {
            MatchResult::ContinueWith(
                word.pos,
                Types::Void,
                Box::new(alias::NoneState::new_stat()),
            )
        } else {
            MatchResult::ContinueWith(
                word.pos,
                Types::Void,
                Box::new(alias::NoneState::new_stat_cont()),
            )
        }
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<(usize, ReturnType)>,
        word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        if let Expr::ForEach {
            indexes, end, var, ..
        } = env.expr
        {
            if !self.has_list {
                //add child and find stats
                if let Some((index, _)) = child_index {
                    self.has_list = true;
                    indexes.push(index);
                    env.symbols.add_layer();
                    env.symbols.insert_var(var.name.to_owned(), ReturnType::Any);
                    MatchResult::ContinueWith(
                        word.pos,
                        Types::Void,
                        Box::new(alias::NoneState::new_stat_cont()),
                    )
                } else {
                    // if child match fail, I can never succeed
                    MatchResult::Failed
                }
            } else {
                //and stat child
                if let Some((index, return_type)) = child_index {
                    // needs to return void
                    if return_type != ReturnType::Null {
                        self.has_stat = true;
                    }
                    indexes.push(index);
                }
                if word.len() == 0 {
                    env.symbols.remove_layer();
                    MatchResult::Failed
                } else if self.has_stat && is_mandatory_close(word) {
                    // close if have close
                    *end = End::from_slice(&word, env.global_index);
                    env.symbols.remove_layer();
                    MatchResult::Matched(word.pos, ReturnType::Void, true)
                    // succeeded - continue again with noncont stat
                } else if child_index.is_some() {
                    MatchResult::ContinueWith(
                        word.pos,
                        Types::Void,
                        get_state!(alias::NoneState::new_stat()),
                    )
                    // failed - pass word
                } else {
                    MatchResult::Continue(0)
                }
            }
        } else {
            unreachable!()
        }
    }

    fn get_name(&self) -> &'static str {
        "While"
    }

    fn get_type(&self) -> StateType {
        StateType::Stat
    }
}

impl ForEachState {
    pub fn new() -> Self {
        Self {
            first: true,
            has_list: false,
            has_stat: false,
        }
    }
}
