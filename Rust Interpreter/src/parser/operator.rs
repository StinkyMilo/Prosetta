use super::*;

#[derive(Debug)]
pub struct OperatorState {
    last_child_index: usize,
    fn_type: OperatorType,
    cont: bool,
}

impl ParseState for OperatorState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, _rest: &Slice) -> MatchResult {
        if !self.cont {
            let locs = env.locs.take().unwrap_or_default();
            *env.expr = Expr::Operator {
                func_type: self.fn_type,
                locs,
                indexes: Vec::new(),
            };
            self.last_child_index = env.child_index;
        }
        // setup child state
        MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_expr()))
    }

    fn step_match(
        &mut self,
        env: &mut Enviroment,
        did_child_match: bool,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        // if child matched - find next child
        if did_child_match {
            self.add_child(env.expr, self.last_child_index);
            self.last_child_index = env.child_index;
            MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_expr()))
        // if word contains h - end
        } else if find_close(word, 0).is_some() {
            // if has 2 or more children - match otherwise fail
            if self.get_child_count(env.expr) >= 2 {
                MatchResult::Matched(rest.pos)
            } else {
                MatchResult::Failed
            }
        // if no h - continue
        } else {
            self.cont = true;
            MatchResult::Continue
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
            last_child_index: usize::MAX,
            fn_type,
            cont: false,
        }
    }

    fn add_child(&self, expr: &mut Expr, child_index: usize) {
        match expr {
            Expr::Operator { indexes, .. } => indexes.push(child_index),
            _ => unimplemented!(),
        }
    }
    fn get_child_count(&self, expr: &mut Expr) -> usize {
        match expr {
            Expr::Operator { indexes, .. } => indexes.len(),
            _ => unimplemented!(),
        }
    }
}

impl OperatorState {
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
