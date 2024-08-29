use super::*;

#[derive(Debug)]

pub struct MultiLitNumState {
    has_data: bool,
    first: bool,
}

impl ParseState for MultiLitNumState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, _rest: &Slice) -> MatchResult {
        if self.first {
            let locs = env.locs.take().unwrap_or_default();
            *env.expr = Expr::MultiLitNum {
                locs,
                end: usize::MAX,
                num_indexes: Vec::new(),
            };
        }
        MatchResult::ContinueWith(word.pos, Box::new(num_literal::LiteralNumState::new()))
    }

    fn step_match(
        &mut self,
        env: &mut Enviroment,
        child_index: Option<usize>,
        word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        self.first = false;

        // add child if matched
        if let Some(index) = child_index {
            self.has_data = true;
            if let Expr::MultiLitNum { num_indexes, .. } = env.expr {
                num_indexes.push(index);
            }
        }

        // if the word is a close, then close
        if is_close(word) {
            // I have data - I succeed
            if self.has_data {
                if let Expr::MultiLitNum { end, .. } = env.expr {
                    *end = word.pos;
                }
                MatchResult::Matched(word.pos, true)
            } else {
                // I do not have data - I cannot close
                MatchResult::Continue
            }
        // child matched - add new child
        } else if child_index.is_some() {
            MatchResult::ContinueWith(word.pos, Box::new(num_literal::LiteralNumState::new()))
        // child failed - move over word
        } else {
            MatchResult::Continue
        }
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
            has_data: false,
            first: true,
        }
    }
}
