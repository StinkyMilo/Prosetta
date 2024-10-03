use alias::NoneState;

use super::*;

#[derive(Debug)]

pub struct PrintState {
    first: bool
}

impl ParseState for PrintState {
    fn step(&mut self, env: &mut Environment, word: &Slice, _rest: &Slice) -> MatchResult {
        let found_close = is_mandatory_close(word);

        if self.first {
            let mut end = End::none();

            // "pri ." - useful for newline? - can change later
            if found_close {
                end = End::from_slice(&word, env.global_index)
            }

            *env.expr = Expr::Print {
                locs: env.locs.take().unwrap_or_default(),
                indexes: Vec::new(),
                single_word: None,
                end,
            };
        }

        if let Expr::Print { single_word, end, .. } = env.expr {
            if found_close {
                // set end
                *end = End::from_slice(&word, env.global_index);
                MatchResult::Matched(word.pos, true)
            } else {
                //get first word for "pri hi."
                if self.first {
                   *single_word = Some(word.str.to_vec());
                }
                MatchResult::ContinueWith(word.pos, get_state!(NoneState::new_expr_cont()))
            }
        }else{
            unreachable!()
        }
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<usize>,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        self.first = false;
        if let Expr::Print { indexes, end, single_word, .. } = env.expr {
            if let Some(index) = child_index {
                indexes.push(index);
                *single_word = None;
            }
            if is_mandatory_close(word){
                *end = End::from_slice(&word, env.global_index);
                MatchResult::Matched(word.pos, true)
            }else if child_index.is_some(){
                MatchResult::ContinueWith(word.pos, get_state!(alias::NoneState::new_expr()))
            }else{
                MatchResult::Continue
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
        Self {
            first: true
        }
    }
}
