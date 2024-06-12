use super::*;
/// state for equals
#[derive(Debug)]
pub struct EqState {}
impl ParseState for EqState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        // set expr
        *env.expr = Expr::Eq {
            name_start: word.pos + env.global_index,
            name: word.str.to_owned(),
            value_index: env.child_index,
            locs: env.locs.take().unwrap_or_default(),
        };
        // setup child state
        MatchResult::ContinueWith(rest.pos, Box::new(alias::NoneState::new_expr_cont(env)))
    }

    fn step_match(
        &mut self,
        _env: &mut Enviroment,
        did_child_match: bool,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if did_child_match {
            // find closing h to end
            let close = find_h_close(&word, 0).or_else(|| find_h_close(&rest, 0));
            match close {
                // will never be a h to find even on future words
                None => MatchResult::Failed,
                Some(slice) => MatchResult::Matched(slice.pos),
            }
        } else {
            // child expr failed
            // if child match fail, I can never succeed
            MatchResult::Failed
        }
    }

    fn get_name(&self) -> &'static str {
        "Equals"
    }

    fn do_replace(&self) -> bool {
        false
    }
}
impl EqState {
    pub fn new() -> Self {
        EqState {}
    }
}
// fn step_eq(
//     env: &mut Enviroment,
//     result: LastMatchResult,
//     word: &Slice,
//     rest: &Slice,
// ) -> MatchResult {
//     match result {
//         // first time
//         LastMatchResult::None => {
//             let Expr::Eq {
//                 name_start,
//                 name,
//                 value_index,
//                 ..
//             } = env.expr
//             else {
//                 unimplemented!()
//             };
//             *name_start = word.pos;
//             *name = word.str.to_owned();
//             *value_index = env.child_index;
//             // setup expr child
//             MatchResult::Continue(rest.pos, Expr::NoneExpr, StateContext::None)
//         }
//         // child expr matched
//         LastMatchResult::Matched => {
//             let close = find_h_close(&word, 0).or_else(|| find_h_close(&rest, 0));
//             match close {
//                 // will never be a h to find even on future words
//                 None => MatchResult::Failed,
//                 Some(slice) => {
//                     // let Expr::Eq {  .. } = env.expr else {
//                     //     unimplemented!()
//                     // };
//                     MatchResult::Matched(slice.pos)
//                 }
//             }
//         }
//         // child expr failed
//         // if child match fail, I can never succeed
//         LastMatchResult::Failed => MatchResult::Failed,
//     }
// }
