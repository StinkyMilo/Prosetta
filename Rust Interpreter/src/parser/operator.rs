use super::*;

#[derive(Debug)]
pub struct OperatorState {
    fn_type: OperatorType,
    first: bool,
    child_count: usize,
}

impl ParseState for OperatorState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, _rest: &Slice) -> MatchResult {
        if self.first {
            *env.expr = Expr::Operator {
                locs: env.locs.take().unwrap_or_default(),
                func_type: self.fn_type,
                indexes: Vec::new(),
                end: usize::MAX,
            };
        }
        // setup child state
        MatchResult::ContinueWith(word.pos, get_state!(alias::NoneState::new_expr()))
    }

    fn step_match(
        &mut self,
        env: &mut Enviroment,
        child_index: Option<usize>,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        self.first = false;
        if let Expr::Operator { indexes, end, .. } = env.expr {
            // add child if matched
            if let Some(index) = child_index {
                self.child_count += 1;
                indexes.push(index);
            }

            // can't have more children
            if fn_type_in_range(self.fn_type, self.child_count)
                && !fn_type_in_range(self.fn_type, self.child_count + 1)
            {
                let close = find_close(&word, 0).or_else(|| find_close(&rest, 0));
                return match close {
                    // will never be a period to find even on future words
                    None => MatchResult::Failed,
                    Some(slice) => {
                        *end = slice.pos + env.global_index;
                        MatchResult::Matched(slice.pos + 1)
                    }
                };
            // on close
            } else if is_close(word) {
                // return if has enough children?
                if fn_type_in_range(self.fn_type, self.child_count) {
                    *end = word.pos + env.global_index;
                    return MatchResult::Matched(word.pos + 1);
                }
            }
            // move next
            if child_index.is_some() {
                MatchResult::ContinueWith(word.pos, get_state!(alias::NoneState::new_expr()))
            } else {
                MatchResult::Continue
            }
        // child is not operator - should not be possible
        } else {
            unreachable!()
        }
    }

    fn get_name(&self) -> &'static str {
        match self.fn_type {
            OperatorType::Add => "Add",
            OperatorType::Mult => "Mult",
            OperatorType::Sub => "Sub",
            OperatorType::Div => "Div",
            OperatorType::Mod => "Mod",
            OperatorType::Exp => "Exp",
            OperatorType::Log => "Log",
        }
    }
    fn do_replace(&self) -> bool {
        false
    }
}

impl OperatorState {
    fn new(fn_type: OperatorType) -> Self {
        OperatorState {
            fn_type,
            first: true,
            child_count: 0,
        }
    }

    pub fn new_add() -> Self {
        Self::new(OperatorType::Add)
    }

    pub fn new_sub() -> Self {
        Self::new(OperatorType::Sub)
    }

    pub fn new_mult() -> Self {
        Self::new(OperatorType::Mult)
    }

    pub fn new_div() -> Self {
        Self::new(OperatorType::Div)
    }

    pub fn new_mod() -> Self {
        Self::new(OperatorType::Mod)
    }

    pub fn new_exp() -> Self {
        Self::new(OperatorType::Exp)
    }

    pub fn new_log() -> Self {
        Self::new(OperatorType::Log)
    }
}

fn fn_type_in_range(fn_type: OperatorType, num: usize) -> bool {
    match fn_type {
        OperatorType::Add => num >= 2,
        OperatorType::Sub => num == 1 || num == 2,
        OperatorType::Mult => num >= 2,
        OperatorType::Div => num == 2,
        OperatorType::Mod => num == 2,
        OperatorType::Exp => num == 2,
        OperatorType::Log => num == 1 || num == 2,
    }
}
