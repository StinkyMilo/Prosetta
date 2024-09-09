use num_literal::get_number_word;

use super::*;

pub enum VarOrInt{
    Var(Vec<u8>),
    Int(i64)
}

#[derive(Debug)]
pub struct MultiLitNumState {
    first: bool,
}

impl ParseState for MultiLitNumState {
    fn step(&mut self, env: &mut Environment, word: &Slice, _rest: &Slice) -> MatchResult {
        if self.first {
            let locs = env.locs.take().unwrap_or_default();
            self.first=false;
            *env.expr = Expr::MultiLitNum {
                locs,
                end: End::none(),
                values: Vec::new(),
            };
            MatchResult::Continue
        }else {
            if let Expr::MultiLitNum { locs, values, end } = env.expr {
                if is_close(word){
                    *end = End::from_slice(&word, env.global_index);
                    MatchResult::Matched(word.pos, true)
                }else{
                    let lower = word.str.to_ascii_lowercase();
                    if env.vars.contains(lower.clone()) {
                        values.push(VarOrInt::Var(lower));
                    } else if let Some(num_value) = get_number_word(word.str){
                        values.push(VarOrInt::Int(num_value%10));
                    }
                    MatchResult::Continue
                }
            }else{
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
        unreachable!()
        // self.first = false;

        // // add child if matched
        // if let Some(index) = child_index {
        //     self.has_data = true;
        //     if let Expr::MultiLitNum { num_indexes, .. } = env.expr {
        //         num_indexes.push(index);
        //     } else {
        //         unreachable!()
        //     }
        // }

        // // if the word is a close, then close
        // if is_close(word) {
        //     // I have data - I succeed
        //     if self.has_data {
        //         if let Expr::MultiLitNum { end, .. } = env.expr {
        //             *end = End::from_slice(&word, env.global_index);
        //         } else {
        //             unreachable!()
        //         }
        //         MatchResult::Matched(word.pos, true)
        //     } else {
        //         // I do not have data - I cannot close
        //         MatchResult::Continue
        //     }
        // // child matched - add new child
        // } else if child_index.is_some() {
        //     MatchResult::ContinueWith(word.pos, Box::new(num_literal::LiteralNumState::new()))
        // // child failed - move over word
        // } else {
        //     MatchResult::Continue
        // }
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
        Self {
            first: true,
        }
    }
}
