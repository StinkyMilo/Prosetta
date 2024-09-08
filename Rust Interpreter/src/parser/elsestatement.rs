use super::*;
/// state for equals
#[derive(Debug)]
pub struct ElseState {
    first: bool,
}
impl ParseState for ElseState {
    fn step(&mut self, env: &mut Environment, word: &Slice, _rest: &Slice) -> MatchResult {
        if let Some(Expr::If { .. }) = env.last_matched_expr {
            if self.first {
                *env.expr = Expr::Else {
                    locs: env.locs.take().unwrap_or_default(),
                    indexes: Vec::new(),
                    end: End::none(),
                };
                // go up layer
                env.vars.add_layer();
            }
            // non cont stat for seeing closes
            MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_stat()))
        } else {
            MatchResult::Failed
        }
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<usize>,
        word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        self.first = false;
        //if let Some(Expr::If { else_index, .. }) = env.last_matched_expr {
        if let Expr::Else { end, indexes, .. } = env.expr {
            if let Some(index) = child_index {
                indexes.push(index);
            }

            // close if have close
            if is_close(word) {
                *end = End::from_slice(&word, env.global_index);
                *else_index = env.expr_index;
                env.vars.remove_layer();
                MatchResult::Matched(word.pos, true)
                // succeeded - continue again with noncont stat
            } else if child_index.is_some() {
                MatchResult::ContinueWith(word.pos, get_state!(alias::NoneState::new_stat()))
                // failed - pass word
            } else {
                MatchResult::Continue
            }
        } else {
            unreachable!()
        }
        // "if" was not previous state
        // } else {
        //     MatchResult::Failed
        // }
    }

    fn get_name(&self) -> &'static str {
        "Else"
    }

    fn do_replace(&self) -> bool {
        false
    }
}

impl ElseState {
    pub fn new() -> Self {
        Self { first: true }
    }
}
