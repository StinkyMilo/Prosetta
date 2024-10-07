use std::usize;

use super::*;

#[derive(Debug, PartialEq)]
pub enum VarOrStr {
    Var(SubStrData),
    Str(Vec<u8>)
}
#[derive(Debug)]
pub struct LitStrState {first: bool, current_str_start: usize, current_str_end: usize}

impl ParseState for LitStrState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        // let g = 0;
        // black_box(&g);
        // println!("G{}", g);
        
        if word.len() == 1 && word.str[0] == b'"'{
            if self.first {
                self.first=false;
                *env.expr = Expr::LitString {
                    str_start: word.pos + env.global_index,
                    str:Vec::new(),
                    str_end: usize::MAX
                };
                self.current_str_start=word.pos + env.global_index + 1;
                MatchResult::Continue
            } else {
                if let Expr::LitString { str, str_end, .. } = env.expr{
                    //Add current str
                    self.current_str_end = word.pos + env.global_index;
                    if self.current_str_end > self.current_str_start {
                        str.push(VarOrStr::Str(env.full_text[self.current_str_start..self.current_str_end].to_vec()));
                    }
                    *str_end = self.current_str_end;
                    MatchResult::Matched(rest.pos, false)
                }else{
                    unreachable!()
                }
            }
        } else {
            if self.first {
                MatchResult::Failed
            } else {
                if let Expr::LitString { str, .. } = env.expr{
                    if let Some(var) = env.vars.try_get_var(word, env.global_index){
                        self.current_str_end = word.pos + env.global_index;
                        if self.current_str_end > self.current_str_start {
                            str.push(VarOrStr::Str(env.full_text[self.current_str_start..self.current_str_end].to_vec()));
                        }
                        str.push(VarOrStr::Var(var));
                        self.current_str_start = rest.pos;
                    }
                    MatchResult::Continue
                }else{
                    unreachable!()
                }
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
        Self {first: true, current_str_start: usize::MAX, current_str_end: usize::MAX}
    }
}

//pri hi. pri hello world. pri "hello world".
