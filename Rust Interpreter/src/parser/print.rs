use alias::NoneState;

use super::*;

#[derive(Debug)]

pub struct PrintState {
    count: usize,
}

impl ParseState for PrintState {
    fn step(&mut self, env: &mut Environment, word: &Slice, _rest: &Slice) -> MatchResult {
        let found_close = is_mandatory_close(word);

        if self.count == 0 {
            let mut end = End::none();

            // "pri ." - useful for newline? - can change later
            if found_close {
                end = End::from_slice(&word, env.global_index)
            }

            *env.expr = Expr::Print {
                locs: env.locs.take().unwrap_or_default(),
                indexes: Vec::new(),
                single_word: None,
                single_word_start: usize::MAX,
                end,
            };
        }

        if let Expr::Print {
            single_word,
            end,
            single_word_start,
            ..
        } = env.expr
        {
            if found_close {
                // set end
                *end = End::from_slice(&word, env.global_index);
                MatchResult::Matched(word.pos, ReturnType::Void, true)
            } else {
                //get first word for "pri hi."
                if self.count == 0 {
                    *single_word = Some(word.str.to_vec());
                    *single_word_start = word.pos;
                }
                self.count += 1;
                MatchResult::ContinueWith(word.pos, Types::Any, get_state!(NoneState::new_expr()))
            }
        } else {
            unreachable!()
        }
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<(usize, ReturnType)>,
        word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        if let Expr::Print {
            indexes,
            end,
            single_word,
            ..
        } = env.expr
        {
            if let Some((index, _)) = child_index {
                indexes.push(index);
                *single_word = None;
            } else if self.count == 1 {
                self.count += 1;
                return MatchResult::Continue(0);
            } else {
                *single_word = None;
            }
            self.count += 1;
            if is_mandatory_close(word) {
                *end = End::from_slice(&word, env.global_index);
                MatchResult::Matched(word.pos, ReturnType::Void, true)
            } else if child_index.is_some() {
                MatchResult::ContinueWith(
                    word.pos,
                    Types::Any,
                    get_state!(alias::NoneState::new_expr()),
                )
            } else {
                MatchResult::Continue(0)
            }
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
        Self { count: 0 }
    }
}
