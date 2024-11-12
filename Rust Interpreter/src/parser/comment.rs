use std::usize;

use super::*;

#[derive(Debug)]
pub struct CommentState {}

impl ParseState for CommentState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        // let g = 0;
        // black_box(&g);
        // println!("G{}", g);
        if word.len() == 1 && word.str[0] == b'['{
            if let Some(close) = rest.str.find("]"){
                *env.expr = Expr::Comment {
                    start: word.pos + env.global_index,
                    comment:rest.str[0..close].to_vec(),
                    end: close + rest.pos + env.global_index
                };
                MatchResult::Matched(rest.pos + close, false)
            }else{
                MatchResult::Failed
            }
        } else {
            MatchResult::Failed
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

impl CommentState {
    pub fn new() -> Self {
        Self {}
    }
}

//pri hi. pri hello world. pri "hello world".
