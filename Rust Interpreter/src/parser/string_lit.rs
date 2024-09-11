use super::*;
#[derive(Debug)]

pub struct LitStrState {}

impl ParseState for LitStrState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        // let g = 0;
        // black_box(&g);
        // println!("G{}", g);
        if word.len() == 1 && (word.str[0] == b'"' || word.str[0] == b'\'') {
            let char = word.str[0];
            let end = rest.str.iter().position(|c| *c == char);
            if let Some(end_pos) = end {
                let end = end_pos + rest.pos;
                //println!("", g);
                let str = rest.str[..end_pos].to_vec();
                //println!("{}", String::from_utf8_lossy(&str));
                *env.expr = Expr::LitString {
                    str_start: word.pos + env.global_index,
                    str,
                };
                MatchResult::Matched(end+1, false)
            } else {
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

    fn do_replace(&self) -> bool {
        false
    }
}

impl LitStrState {
    pub fn new() -> Self {
        Self {}
    }
}

//pri hi. pri hello world. pri "hello world".
