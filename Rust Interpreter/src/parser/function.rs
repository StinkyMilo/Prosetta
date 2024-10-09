use super::*;
/// state for equals
#[derive(Debug)]
pub struct FunctionState {
    first: bool,
    has_name: bool,
    has_args: bool,
}
impl ParseState for FunctionState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        if self.first {
            *env.expr = Expr::Function {
                name_start: usize::MAX,
                name: Vec::new(),
                locs: env.locs.take().unwrap_or_default(),
                indexes: Vec::new(),
                arg_starts: Vec::new(),
                arg_names: Vec::new(),
                end: End::none(),
            };
            self.first = false;
            // setup child state
            // MatchResult::ContinueWith(rest.pos, Box::new(alias::NoneState::new_expr_cont()))
        }
        if !self.has_name {
            if is_close(word) || (word.len() > 0 && (word.str[0] == b'"' || word.str[0] == b'\'')) {
                MatchResult::Continue
            } else {
                if let Expr::Function {
                    name_start, name, ..
                } = env.expr
                {
                    *name_start = word.pos + env.global_index;
                    let temp_name = word.str.to_ascii_lowercase();
                    if temp_name.len() < 3 {
                        return MatchResult::Failed;
                    }
                    *name = temp_name.to_owned();
                    env.funcs.insert(temp_name.to_owned(), 0);
                    env.vars.add_layer();
                } else {
                    unreachable!()
                }
                self.has_name = true;
                MatchResult::Continue
            }
        } else if !self.has_args {
            if is_mandatory_close(word) {
                self.has_args = true;
                env.funcs.add_layer();
                MatchResult::ContinueWith(rest.pos, Box::new(alias::NoneState::new_stat_cont()))
            } else if is_close(word) {
                MatchResult::Continue
            } else {
                if let Expr::Function {
                    name,
                    arg_starts,
                    arg_names,
                    ..
                } = env.expr
                {
                    let arg_name = word.str.to_ascii_lowercase();
                    if arg_name.len() < 3 {
                        return MatchResult::Continue;
                    }
                    arg_starts.push(word.pos + env.global_index);
                    arg_names.push(arg_name.to_owned());
                    env.vars.insert(arg_name.to_owned());
                    env.funcs.inc_arg_count(name);
                }
                MatchResult::Continue
            }
        } else {
            MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_stat_cont()))
        }
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<usize>,
        word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        if let Expr::Function { indexes, end, .. } = env.expr {
            //and stat child
            if let Some(index) = child_index {
                indexes.push(index);
            }

            // close if have close
            if is_close(word) {
                *end = End::from_slice(&word, env.global_index);
                env.remove_var_layer();
                MatchResult::Matched(word.pos, true)
                // succeeded - continue again with noncont stat
            } else if child_index.is_some() {
                MatchResult::ContinueWith(word.pos, get_state!(alias::NoneState::new_stat_cont()))
                // failed - pass word
            } else {
                MatchResult::Continue
            }
        } else {
            unreachable!()
        }
    }

    fn get_name(&self) -> &'static str {
        "Function"
    }

    fn get_type(&self) -> StateType {
        StateType::Stat
    }
}

impl FunctionState {
    pub fn new() -> Self {
        Self {
            first: true,
            has_name: false,
            has_args: false,
        }
    }
}
