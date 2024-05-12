use super::*;

#[derive(Debug)]

pub struct LitNumState {
    first: bool,
}

impl ParseState for LitNumState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        // check if number
        if let Some(new_digit) = num_literal::get_number_word(word.str) {
            self.first = true;

            //set up or update command
            if let Expr::LitNum {
                str_start,
                str_length,
                value,
                ..
            } = env.expr
            {
                *value = *value * 10 + new_digit;
                *str_length = word.end() - *str_start - env.global_index;
            } else {
                let locs = env.locs.take().unwrap_or_default();
                *env.expr = Expr::LitNum {
                    locs,
                    str_start: word.pos + env.global_index,
                    str_length: word.len(),
                    value: new_digit,
                };
            }
            MatchResult::Continue
            //if not number, check h
        } else {
            let close = find_h_close(&word, 0).or_else(|| find_h_close(&rest, 0));
            match close {
                // will never be a h to find even on future words
                None => MatchResult::Failed,
                Some(slice) => MatchResult::Matched(slice.pos),
            }
        }
    }

    fn step_match(
        &mut self,
        _env: &mut Enviroment,
        _did_child_match: bool,
        _word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        // has no child to match - fn should never be called
        unimplemented!()
    }

    fn get_name(&self) -> &'static str {
        "NumLit"
    }

    fn do_replace(&self) -> bool {
        false
    }
}

impl LitNumState {
    pub fn new() -> Self {
        LitNumState { first: false }
    }
}
