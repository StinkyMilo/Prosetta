use super::*;
/// state for equals
#[derive(Debug)]
pub struct FunctionState {
    first: bool,
    has_name: bool,
    is_parsing_body: bool,
    has_stat: bool,
    args_count: u8,
}
impl ParseState for FunctionState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        if self.first {
            *env.expr = Expr::Function {
                locs: env.locs.take().unwrap_or_default(),
                func: SubStrData::new(),
                args: Vec::new(),
                indexes: Vec::new(),
                end: End::none(),
            };
            self.first = false;
        }

        // reached the end of the string
        if word.len() == 0 {
            env.symbols.remove_layer();
            MatchResult::Failed
        } else if let Expr::Function { func, args, .. } = env.expr {
            // if parsing stats in body
            if self.is_parsing_body {
                MatchResult::ContinueWith(rest.pos, Box::new(alias::NoneState::new_stat()))
            // if doesn't yet have name
            } else if !self.has_name {
                if let Some(func_data) = try_get_symbol_word(word, env.global_index) {
                    *func = func_data;
                    //put func into new layer
                    env.symbols.add_layer();
                    env.symbols.insert_func(func.name.to_owned(), 0);
                    self.has_name = true;
                }
                MatchResult::Continue(0)
            //if doesn't yet have arg
            } else {
                if is_mandatory_close(word) {
                    self.is_parsing_body = true;
                    env.symbols
                        .insert_func(func.name.to_owned(), self.args_count);
                    return MatchResult::ContinueWith(
                        rest.pos,
                        Box::new(alias::NoneState::new_stat_cont()),
                    );
                    // if word can be a varible
                } else if let Some(arg_data) = try_get_symbol_word(word, env.global_index) {
                    // funtion can only have 255 arguments
                    if self.args_count != 255 && !env.symbols.contains(&arg_data.name) {
                        env.symbols.insert_var(arg_data.name.to_owned());
                        args.push(arg_data);
                        self.args_count += 1;
                    }
                    //let arg_name = word.str.to_ascii_lowercase();
                    // arg_names.push(arg_name.to_owned());
                    // env.vars.insert(arg_name.to_owned());
                    //env.funcs.inc_arg_count(&func.name);
                }
                MatchResult::Continue(0)
            }
        } else {
            unreachable!()
        }
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<usize>,
        word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        // reached the end of the string
        if word.len() == 0 {
            env.symbols.remove_layer();
            MatchResult::Failed
        } else if let Expr::Function {
            func, indexes, end, ..
        } = env.expr
        {
            //and stat child
            if let Some(index) = child_index {
                indexes.push(index);
            }

            // close if have close
            if is_mandatory_close(word) {
                *end = End::from_slice(&word, env.global_index);
                env.symbols.remove_layer();
                env.symbols
                    .insert_func(func.name.to_owned(), self.args_count);
                MatchResult::Matched(word.pos, ReturnType::Void, true)
                // succeeded - continue again with noncont stat
            } else if child_index.is_some() {
                MatchResult::ContinueWith(word.pos, get_state!(alias::NoneState::new_stat_cont()))
                // failed - pass word
            } else {
                MatchResult::Continue(0)
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
            is_parsing_body: false,
            has_stat: false,
            args_count: 0,
        }
    }
}
