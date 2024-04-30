use super::*;

#[derive(Debug)]
pub struct CircleState {
    children: u8,
}
impl ParseState for CircleState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, _rest: &Slice) -> MatchResult {
        let locs = env.locs.take().unwrap_or_default();

        *env.expr = Expr::Circle {
            locs,
            x_index: env.child_index,
            y_index: usize::MAX,
            r_index: usize::MAX,
        };

        // setup child state
        MatchResult::ContinueWith(word.pos, Box::new(builtins::NoneState::new_expr()))
    }

    fn step_match(
        &mut self,
        env: &mut Enviroment,
        did_child_match: bool,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if did_child_match {
            if self.children == 2 {
                // matched second child - find h
                let close = find_h_close(&word, 0).or_else(|| find_h_close(&rest, 0));
                match close {
                    // will never be a h to find even on future words
                    None => MatchResult::Failed,
                    Some(slice) => MatchResult::Matched(slice.pos),
                }
            } else {
                // matched first child - setup second child
                self.set_child_indexes(env.expr, self.children, env.child_index);
                self.children += 1;
                MatchResult::ContinueWith(word.pos, Box::new(builtins::NoneState::new_expr()))
            }
        } else {
            // if either child match fails - I will never match
            MatchResult::Failed
        }
    }

    fn get_name(&self) -> &'static str {
        "Circle"
    }

    fn do_replace(&self) -> bool {
        false
    }
}

impl CircleState {
    pub fn new() -> Self {
        CircleState { children: 0 }
    }
}

impl CircleState {
    fn set_child_indexes(&self, expr: &mut Expr, field_index: u8, child_index: usize) {
        match expr {
            Expr::Circle {
                y_index, r_index, ..
            } => match field_index {
                0 => *y_index = child_index,
                1 => *r_index = child_index,
                _ => {
                    unimplemented!()
                }
            },
            _ => {
                unimplemented!()
            }
        }
    }
}
