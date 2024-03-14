use super::*;

#[derive(Debug)]
pub enum BiFunctionType {
    Add,
    Mult,
}

#[derive(Debug)]
pub struct BiFuncState {
    has_matched_first: bool,
    fn_type: BiFunctionType,
}
impl ParseState for BiFuncState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, _rest: &Slice) -> MatchResult {
        let locs = env.locs.take().unwrap_or_default();
        *env.expr = match self.fn_type {
            BiFunctionType::Add => Expr::Add {
                locs,
                a_index: env.child_index,
                b_index: usize::MAX,
            },
            BiFunctionType::Mult => Expr::Mult {
                locs,
                a_index: env.child_index,
                b_index: usize::MAX,
            },
        };

        // setup child state
        MatchResult::Continue(word.pos, Box::new(builtins::NoneState::new_expr()))
    }

    fn step_match(
        &mut self,
        env: &mut Enviroment,
        did_child_match: bool,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if did_child_match {
            if self.has_matched_first {
                // matched second child - find h
                let close = find_h_close(&word, 0).or_else(|| find_h_close(&rest, 0));
                match close {
                    // will never be a h to find even on future words
                    None => MatchResult::Failed,
                    Some(slice) => {
                        MatchResult::Matched(slice.pos)
                    }
                }
            } else {
                // matched first child - setup second child
                self.has_matched_first = true;
                self.set_b_index(env.expr,env.child_index);
                MatchResult::Continue(word.pos, Box::new(builtins::NoneState::new_expr()))
            }
        } else {
            // if either child match fails - I will never match
            MatchResult::Failed
        }
    }

    fn get_name(&self) -> &'static str {
        match self.fn_type {
            BiFunctionType::Add => "Add",
            BiFunctionType::Mult => "Mult",
        }
    }
    fn do_replace(&self) -> bool {
        false
    }
}
impl BiFuncState {
    fn new(fn_type: BiFunctionType) -> Self {
        BiFuncState {
            has_matched_first: false,
            fn_type,
        }
    }

    pub fn new_add() -> Self {
        Self::new(BiFunctionType::Add)
    }

    pub fn new_mult() -> Self {
        Self::new(BiFunctionType::Mult)
    }

    fn set_b_index(&self,expr:&mut Expr,child_index:usize){
        match expr {
            Expr::Add{b_index,..}|Expr::Mult {  b_index,.. }=>{
                *b_index=child_index;
            },
            _=>{unimplemented!()}
        }
    }
}
