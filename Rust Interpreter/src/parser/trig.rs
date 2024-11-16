use super::*;

#[derive(Debug)]
pub struct TrigState {
    fn_type: TrigType,
}

impl ParseState for TrigState {
    fn step(&mut self, env: &mut Environment, word: &Slice, _rest: &Slice) -> MatchResult {
        // set expr
        *env.expr = Expr::Trig {
            locs: env.locs.take().unwrap_or_default(),
            func_type: self.fn_type,
            index: usize::MAX,
            end: End::none(),
        };

        // setup child state
        MatchResult::ContinueWith(
            word.pos,
            Types::Number,
            Box::new(alias::NoneState::new_expr_cont()),
        )
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<(usize, ReturnType)>,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if let Some((ind, _)) = child_index {
            // find ending close
            let close = find_close_slice(&word, 0).or_else(|| find_close_slice(&rest, 0));
            match close {
                // will never be a period to find even on future words
                None => MatchResult::Failed,
                Some(slice) => {
                    if let Expr::Trig { index, end, .. } = env.expr {
                        *index = ind;
                        *end = End::from_slice(&slice.0, env.global_index);
                    } else {
                        unreachable!();
                    }
                    MatchResult::Matched(slice.0.pos, ReturnType::Number, true)
                }
            }
        } else {
            // child expr failed
            // if child match fail, I can never succeed
            MatchResult::Failed
        }
    }
    fn get_name(&self) -> &'static str {
        match self.fn_type {
            TrigType::Sin => "Sin",
            TrigType::Cos => "Cos",
            TrigType::Tan => "Tan",
        }
    }

    fn get_type(&self) -> StateType {
        StateType::Expr
    }
}

impl TrigState {
    fn new(fn_type: TrigType) -> Self {
        Self { fn_type }
    }

    pub fn new_sin() -> Self {
        Self::new(TrigType::Sin)
    }

    pub fn new_cos() -> Self {
        Self::new(TrigType::Cos)
    }

    pub fn new_tan() -> Self {
        Self::new(TrigType::Tan)
    }
}
