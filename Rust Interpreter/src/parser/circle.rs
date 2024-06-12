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
            indexes: [env.child_index, usize::MAX, usize::MAX],
        };

        // setup child state
        MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_expr_cont()))
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
                self.children += 1;
                self.add_child(env.expr, self.children, env.child_index);
                MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_expr_cont()))
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
    fn add_child(&self, expr: &mut Expr, field_index: u8, child_index: usize) {
        match expr {
            Expr::Circle {
                indexes,..
            } => //match field_index {
            //     0 => *y_index = child_index,
            //     1 => *r_index = child_index,
            //     _ => {
            //         unimplemented!()
            //     }
                indexes[field_index as usize]=child_index,
            _ => {
                unimplemented!()
            }
        }
    }
}
