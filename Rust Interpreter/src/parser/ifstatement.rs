use super::*;
/// state for equals
#[derive(Debug)]
pub struct IfState;
impl ParseState for IfState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        // set expr
        *env.expr = Expr::If {
            condition_start: word.pos + env.global_index,
            locs: env.locs.take().unwrap_or_default(),
            body_start: usize::MAX,
            body_end: usize::MAX,
            has_condition: false
        };
        // setup child state
        MatchResult::ContinueWith(rest.pos, Box::new(alias::NoneState::new_expr_cont()))
    }

    fn step_match(
        &mut self,
        env: &mut Enviroment,
        child_index: Option<usize>,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if let Expr::If {body_start, body_end, has_condition, ..} = env.expr {
            let close = find_close(&word, 0).or_else(|| find_close(&rest, 0));
            if !(*has_condition) {
                match close {
                    None => {
                        MatchResult::Failed
                    },
                    Some(slice) => {
                        *has_condition=true;
                        *body_start = slice.pos + env.global_index;
                        MatchResult::ContinueWith(slice.pos, Box::new(alias::NoneState::new_expr()))
                    }
                }
            }else{
                //If we get a punctuation before an expression, we want to end. Otherwise, we want to continue with a new expression
            }
        }else{
            MatchResult::Failed
        }


        // if let Some(index) = child_index {
        //     // find ending close
        //     let close = find_close(&word, 0).or_else(|| find_close(&rest, 0));
        //     match close {
        //         // will never be a period to find even on future words
        //         None => MatchResult::Failed,
        //         Some(slice) => {
        //             if let Expr::Assign {
        //                 value_index, end, ..
        //             } = env.expr
        //             {
        //                 *value_index = index;
        //                 *end = slice.pos + env.global_index;
        //             }
        //             MatchResult::Matched(slice.pos, true)
        //         }
        //     }
        // } else {
        //     // child expr failed
        //     // if child match fail, I can never succeed
        //     MatchResult::Failed
        // }
    }

    fn get_name(&self) -> &'static str {
        "Assign"
    }

    fn do_replace(&self) -> bool {
        false
    }
}

impl AssignState {
    pub fn new() -> Self {
        Self
    }
}
