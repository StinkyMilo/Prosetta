use super::*;

#[derive(Debug)]

pub struct PrintState {
    first: bool,
    parsing_var: bool,
    first_word: Option<(Vec<u8>, usize)>,
}

impl ParseState for PrintState {
    fn step(&mut self, env: &mut Environment, word: &Slice, _rest: &Slice) -> MatchResult {
        let found_close = is_close(word);

        if self.first {
            let mut end = End::none();

            // "pri ." - useful for newline? - can change later
            if found_close {
                end = End::from_slice(&word, env.global_index)
            }

            *env.expr = Expr::Print {
                locs: env.locs.take().unwrap_or_default(),
                data: Vec::new(),
                end,
            };
        }

        if found_close {
            if let Expr::Print { data, end, .. } = env.expr {
                // is "pri hi." then add word
                if let Some((str, pos)) = self.first_word.take() {
                    data.push(Prints::Word(str, pos))
                }
                // set end
                *end = End::from_slice(&word, env.global_index);
            } else {
                unreachable!()
            }
            MatchResult::Matched(word.pos, true)
        } else {
            //get first word for "pri hi."
            if self.first {
                self.first_word = Some((word.str.to_vec(), word.pos));
            } else {
                self.first_word = None;
            }
            MatchResult::ContinueWith(word.pos, get_state!(var::VarState::new()))
        }
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<usize>,
        word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        self.first = false;
        if let Expr::Print { data, end, .. } = env.expr {
            // prev word was var
            if let Some(index) = child_index {
                //word matched -- first_word now invalid
                self.first_word = None;
                if self.parsing_var {
                    data.push(Prints::Var(index));
                } else {
                    data.push(Prints::String(index));
                }
                if is_mandatory_close(word) {
                    *end = End::from_slice(&word, env.global_index);
                    MatchResult::Matched(word.pos, true)
                } else {
                    self.parsing_var = true;
                    MatchResult::ContinueWith(word.pos, get_state!(var::VarState::new()))
                }
                // curr word was not var
            } else {
                if self.parsing_var {
                    self.parsing_var = false;
                    MatchResult::ContinueWith(word.pos, get_state!(string_lit::LitStrState::new()))
                } else {
                    self.parsing_var = true;
                    MatchResult::Continue
                }
            }
        // data.push(Prints::Word(
        //     word.str.to_owned(),
        //     word.pos + env.global_index,
        // ));
        // MatchResult::Continue
        // if is_close(word) {
        //     *end = word.pos;
        //     MatchResult::Matched(word.pos + 1)
        // } else {
        //     MatchResult::ContinueWith(word.pos, get_state!(var::VarState::new()))
        // }
        } else {
            unreachable!()
        }
    }

    fn get_name(&self) -> &'static str {
        "Print"
    }

    fn get_type(&self) -> StateType {
        StateType::Stat
    }
}

impl PrintState {
    pub fn new() -> Self {
        Self {
            first: true,
            parsing_var: true,
            first_word: None,
        }
    }
}
