use super::*;

#[derive(Debug)]

pub struct PrintState {
    first: bool,
}

impl ParseState for PrintState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, _rest: &Slice) -> MatchResult {
        let matched = is_close(word);

        if self.first {
            let mut end = usize::MAX;

            // "pri ." - useful for newline? - can change later
            if matched {
                end = word.pos + env.global_index;
            }

            *env.expr = Expr::Print {
                locs: env.locs.take().unwrap_or_default(),
                data: Vec::new(),
                end,
            };
        }

        if matched {
            MatchResult::Matched(word.pos, true)
        } else {
            MatchResult::ContinueWith(word.pos, get_state!(var::VarState::new()))
        }
    }

    fn step_match(
        &mut self,
        env: &mut Enviroment,
        child_index: Option<usize>,
        word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        self.first = false;
        if let Expr::Print { data, end, .. } = env.expr {
            // prev word was var
            if let Some(index) = child_index {
                data.push(Prints::Var(index));
                if is_close(word) {
                    *end = word.pos + env.global_index;
                    MatchResult::Matched(word.pos,true)
                } else {
                    MatchResult::ContinueWith(word.pos, get_state!(var::VarState::new()))
                }
                // curr word was not var
            } else {
                data.push(Prints::Word(
                    word.str.to_owned(),
                    word.pos + env.global_index,
                ));
                MatchResult::Continue
            }

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

    fn do_replace(&self) -> bool {
        false
    }
}

impl PrintState {
    pub fn new() -> Self {
        Self { first: true }
    }
}
