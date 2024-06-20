use super::*;

#[derive(Debug)]

pub struct MultiLitNumState {
    has_data: bool,
}

impl ParseState for MultiLitNumState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, _rest: &Slice) -> MatchResult {
        // find close
        if is_close(word) {
            if self.has_data {
                if let Expr::MultiLitNum { end, .. } = env.expr {
                    *end = word.pos;
                }
                MatchResult::Matched(word.pos + 1)
            } else {
                MatchResult::Failed
            }
        }
        // else check if number
        else {
            if let Some(new_digit) = num_literal::get_number_word(word.str) {
                self.has_data = true;

                //set up or update command
                if let Expr::MultiLitNum {
                    str_start,
                    str_length,
                    value,
                    ..
                } = env.expr
                {
                    *value = *value * 10 + new_digit;
                    *str_length = word.end() - *str_start + env.global_index;
                } else {
                    let locs = env.locs.take().unwrap_or_default();
                    *env.expr = Expr::MultiLitNum {
                        locs,
                        str_start: word.pos + env.global_index,
                        str_length: word.len(),
                        value: new_digit,
                        end: usize::MAX,
                    };
                }
            };
            MatchResult::Continue
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
        "MultiLitNum"
    }

    fn do_replace(&self) -> bool {
        false
    }
}

impl MultiLitNumState {
    pub fn new() -> Self {
        MultiLitNumState { has_data: false }
    }
}
