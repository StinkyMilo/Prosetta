use super::*;
/// state for equals
#[derive(Debug)]
pub struct IfState{
    has_condition: bool
}
impl ParseState for IfState {

    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        if !self.has_condition {
            *env.expr = Expr::If {
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
        env: &mut Enviroment,
        child_index: Option<usize>,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        if let Expr::If {body_start, body_end, indexes, ..} = env.expr {
            //If we get a punctuation before an expression, we want to end. Otherwise, we want to continue with a new expression
            //Check the next close. Is it after the child expression? If so, don't even add the child and fail.
            if !(self.has_condition) {
                if let Some(index) = child_index {
                    indexes.push(index);
                    self.has_condition=true;
                    *body_start=index;
                    println!("No condition yet, some expression");
                    MatchResult::ContinueWith(word.pos,get_state!(alias::NoneState::new_stat()))
                }else{
                    //No child
                    println!("No child");
                    MatchResult::Failed
                }
            }else{
                //If we get a punctuation before an expression, we want to end. Otherwise, we want to continue with a new expression
                //Check the next close. Is it after the child expression? If so, don't even add the child and fail.
                let mut statement_found = false;
                if let Some(index) = child_index {
                    indexes.push(index);
                    println!("Statement found");
                    statement_found=true;
                }
                
                if is_close(word){
                    *body_end = word.pos + env.global_index;
                    println!("Close found");
                    MatchResult::Matched(word.pos, true)
                }else if statement_found {
                    println!("Searching for another statement");
                    MatchResult::ContinueWith(word.pos, get_state!(alias::NoneState::new_stat()))
                }else{
                    println!("Nothing found");
                    MatchResult::Continue
                }
            } 
        }else{
            MatchResult::Failed
        }
    }

    fn get_name(&self) -> &'static str {
        "If"
    }

    fn do_replace(&self) -> bool {
        false
    }
}

impl IfState {
    pub fn new() -> Self {
        Self{
            has_condition: false
        }
    }
}
