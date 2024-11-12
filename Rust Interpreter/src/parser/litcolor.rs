use super::*;
use litcolor_data::{get_color_word, LitColorFoundResult};
#[derive(Debug)]

pub struct LiteralColorState {
    wsf: Vec<u8>,
}

impl ParseState for LiteralColorState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        let value = get_color_word(&self.wsf, word.str);
        if self.wsf == b"" {
            match value {
                LitColorFoundResult::Found => {
                    *env.expr = Expr::LitCol {
                        str_start: word.pos + env.global_index,
                        str_length: word.len(),
                        value: word.str.to_owned(),
                    };
                    MatchResult::Matched(rest.pos, ReturnType::Color, false)
                }
                LitColorFoundResult::CouldFind => {
                    *env.expr = Expr::LitCol {
                        str_start: word.pos + env.global_index,
                        str_length: usize::MAX,
                        value: Vec::new(),
                    };
                    self.wsf
                        .append(&mut word.str.to_owned().to_ascii_lowercase());
                    MatchResult::Continue(0)
                }
                LitColorFoundResult::Failed => MatchResult::Failed,
                LitColorFoundResult::FoundOnLast => {
                    unreachable!();
                }
            }
        } else {
            match value {
                //Finishes a color name
                LitColorFoundResult::Found => {
                    if let Expr::LitCol {
                        str_start,
                        str_length,
                        value,
                    } = env.expr
                    {
                        if let Some(len) =
                            (word.pos + word.len() + env.global_index).checked_sub(*str_start)
                        {
                            *str_length = len;
                            self.wsf
                                .append(&mut word.str.to_owned().to_ascii_lowercase());
                            *value = self.wsf.to_owned();
                        } else {
                            unreachable!("Expression is ending before it started!")
                        }
                    } else {
                        unreachable!()
                    }
                    MatchResult::Matched(rest.pos, ReturnType::Color, false)
                }
                //Last word could have had more color words after it but didn't.
                LitColorFoundResult::FoundOnLast => {
                    if let Expr::LitCol {
                        str_start,
                        str_length,
                        value,
                    } = env.expr
                    {
                        if let Some(len) = (word.pos + env.global_index).checked_sub(*str_start) {
                            *str_length = len;
                            *value = self.wsf.to_owned();
                        } else {
                            panic!("Expression is ending before it started!")
                        }
                    } else {
                        unreachable!()
                    }
                    MatchResult::Matched(word.pos, ReturnType::Color, false)
                }
                //Beginning of a color name, keep searching
                LitColorFoundResult::CouldFind => {
                    self.wsf
                        .append(&mut word.str.to_owned().to_ascii_lowercase());
                    MatchResult::Continue(0)
                }
                //Cannot be a color
                LitColorFoundResult::Failed => MatchResult::Failed,
            }
        }
    }

    fn step_match(
        &mut self,
        _env: &mut Environment,
        _child: Option<usize>,
        _word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        unreachable!()
    }

    fn get_name(&self) -> &'static str {
        "ColorLit"
    }

    fn get_type(&self) -> StateType {
        StateType::Expr
    }
}

impl LiteralColorState {
    pub fn new() -> Self {
        Self { wsf: Vec::new() }
    }
}
