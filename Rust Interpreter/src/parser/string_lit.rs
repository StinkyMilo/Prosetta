use std::usize;

use super::*;

#[derive(Debug, PartialEq)]
pub enum VarOrStr {
    Var(SubStrData),
    Str(Vec<u8>),
}
#[derive(Debug)]
pub struct LitStrState {
    first: bool,
    current_str_start: usize,
    current_str_end: usize,
}

impl ParseState for LitStrState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        // found ""
        if word.len() == 1 && word.str[0] == b'"' {
            if self.first {
                self.first = false;
                *env.expr = Expr::LitString {
                    str_start: word.pos + env.global_index,
                    str: Vec::new(),
                    str_end: usize::MAX,
                };
                self.current_str_start = word.pos + 1;
                MatchResult::Continue(0)
                // end string
            } else {
                if let Expr::LitString { str, str_end, .. } = env.expr {
                    //Add current str
                    self.current_str_end = word.pos;
                    if self.current_str_end > self.current_str_start {
                        str.push(VarOrStr::Str(
                            env.full_text[self.current_str_start..self.current_str_end].to_vec(),
                        ));
                    }
                    *str_end = self.current_str_end + env.global_index;
                    MatchResult::Matched(rest.pos, ReturnType::String, false)
                } else {
                    unreachable!()
                }
            }
            // did not find " ever
        } else if self.first {
            MatchResult::Failed
        // did not find "
        } else {
            if let Expr::LitString { str, .. } = env.expr {
                // find var
                if let Some(var) = env.symbols.try_get_var(word, env.global_index) {
                    self.current_str_end = word.pos;
                    if self.current_str_end > self.current_str_start {
                        str.push(VarOrStr::Str(
                            env.full_text[self.current_str_start..self.current_str_end].to_vec(),
                        ));
                    }
                    str.push(VarOrStr::Var(var));
                    self.current_str_start = rest.pos;
                }
                MatchResult::Continue(0)
            } else {
                unreachable!()
            }
        }
    }

    fn step_match(
        &mut self,
        _env: &mut Environment,
        _child_index: Option<usize>,
        _word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        // has no child to match - fn should never be called
        unreachable!()
    }

    fn get_name(&self) -> &'static str {
        "StringLit"
    }

    fn get_type(&self) -> StateType {
        StateType::Expr
    }
}

impl LitStrState {
    pub fn new() -> Self {
        Self {
            first: true,
            current_str_start: usize::MAX,
            current_str_end: usize::MAX,
        }
    }
}

//pri hi. pri hello world. pri "hello world".
