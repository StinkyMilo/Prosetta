use super::*;
/// state for equals
#[derive(Debug)]
pub struct WhileState{
    has_condition: bool
}
impl ParseState for WhileState {

    fn step(&mut self, env: &mut Environment, word: &Slice, _rest: &Slice) -> MatchResult {
        if !self.has_condition {
            env.exprs.vec[env.index] = Expr::While {
                condition_start: word.pos + env.global_index,
                locs: env.locs.take().unwrap_or_default(),
                body_start: usize::MAX,
                indexes:Vec::new(),
                body_end: usize::MAX
            };
            // setup child state
            MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_expr_cont()))
        }else{
            println!("Continuing with new statement");
            MatchResult::ContinueWith(word.pos, Box::new(alias::NoneState::new_stat()))
        }
    }

    fn step_match(
        &mut self,
        env: &mut Environment,
        child_index: Option<usize>,
        word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        if let Expr::While {body_start, body_end, indexes, ..} = &mut env.exprs.vec[env.index] {
            //If we get a punctuation before an expression, we want to end. Otherwise, we want to continue with a new expression
            //Check the next close. Is it after the child expression? If so, don't even add the child and fail.
            if !(self.has_condition) {
                if let Some(index) = child_index {
                    indexes.push(index);
                    self.has_condition=true;
                    *body_start=index;
                    MatchResult::ContinueWith(word.pos,get_state!(alias::NoneState::new_stat()))
                }else{
                    //No child
                    MatchResult::Failed
                }
            }else{
                let mut statement_found = false;
                if let Some(index) = child_index {
                    indexes.push(index);
                    statement_found=true;
                }
                
                if is_close(word){
                    *body_end = word.pos + env.global_index;
                    MatchResult::Matched(word.pos, true)
                }else if statement_found {
                    MatchResult::ContinueWith(word.pos, get_state!(alias::NoneState::new_stat()))
                }else{
                    MatchResult::Continue
                }
            } 
        }else{
            MatchResult::Failed
        }
    }

    fn get_name(&self) -> &'static str {
        "While"
    }

    fn do_replace(&self) -> bool {
        false
    }
}

impl WhileState {
    pub fn new() -> Self {
        Self{
            has_condition: false
        }
    }
}
