use super::*;

/// state for num
#[derive(Debug)]
pub struct NumState {}
impl ParseState for NumState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        let close = find_h_close(rest, 0);
        if let Some(close) = close {
            *env.expr = Expr::Num {
                str_start: word.pos + env.global_index,
                str: word.str.to_owned().to_ascii_lowercase(),
                locs: env.locs.take().unwrap_or_default(),
            };

            return MatchResult::Matched(close.pos);
        }

        // will not work on next word
        MatchResult::Failed
    }

    fn step_match(
        &mut self,
        _env: &mut Enviroment,
        _did_child_match: bool,
        _word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        // has child to match - fn should never be called
        unimplemented!()
    }

    fn get_name(&self) -> &'static str {
        "Num"
    }

    fn do_replace(&self) -> bool {
        false
    }
}
impl NumState {
    pub fn new() -> Self {
        NumState {}
    }
}

// fn step_num(
//     env: &mut Enviroment,
//     _result: LastMatchResult,
//     word: &Slice,
//     rest: &Slice,
// ) -> MatchResult {
//     let close = find_h_close(rest, 0);
//     if let Some(close) = close {
//         let Expr::Num { str_start, str, .. } = env.expr else {
//             unimplemented!()
//         };

//         *str_start = word.pos;
//         *str = word.str.to_owned().to_ascii_lowercase();
//         return MatchResult::Matched(close.pos);
//     }

//     // will not work on next word
//     MatchResult::Failed
// }
